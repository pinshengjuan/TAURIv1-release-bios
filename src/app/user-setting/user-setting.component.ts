import { Component, Output, EventEmitter } from '@angular/core';
import { FormsModule} from '@angular/forms';
import { environment } from '../environments/environment';
import { open } from '@tauri-apps/api/dialog';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faEllipsis} from '@fortawesome/free-solid-svg-icons';
import { appConfigDir } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, readDir, exists  } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';

@Component({
  selector: 'app-user-setting',
  standalone: true,
  imports: [FormsModule, FontAwesomeModule],
  templateUrl: './user-setting.component.html',
  styleUrl: './user-setting.component.css'
})
export class UserSettingComponent {
  faEllipsisH = faEllipsis;
  pathFieldVal = '';
  configFileVal = '';
  defaultPath = '';
  mails: any[] = [
    { address: '', label: 'QT Leader email address'},
    { address: '', label: 'BIOS QT Leader email address'},
  ];
  configFilePath = '';
  configFileMail = '';

  dir = appConfigDir().then((configDir) => {
    this.configFilePath = configDir+'default.path';
    this.configFileMail = configDir+'email.list';
    exists(this.configFilePath).then((sts: boolean)=>{
      if (sts === true) {
        readTextFile(this.configFilePath).then((folder: string)=>{
          if (folder.trim()) {
            invoke<string>('check_folder', { folder }).then( (text) => {
              if (text === 'true') {
                this.pathFieldVal = folder;
                this.configFileVal = folder;
              }
            });
          }
        });
      }
    });
    exists(this.configFileMail).then((sts: boolean)=>{
      if (sts === true) {
        var supervisorMail: string = this.configFileMail.trim()
        invoke<string>('get_qt_address', { supervisorMail }).then( (qt_address) => {
          if (qt_address !== '') {
            this.mails[0].address = qt_address;
          }
        });
        invoke<string>('get_bios_qt_address', { supervisorMail }).then( (bios_qt_address) => {
          if (bios_qt_address !== '') {
            this.mails[1].address = bios_qt_address;
          }
        });
      }
    });
  });



  async pathField() {
    if (this.pathFieldVal) {
      this.defaultPath = this.pathFieldVal;
    } else if (this.configFileVal) {
      this.defaultPath = this.configFileVal;
    }
    const pathFromConfigFile = await open({
      directory: true,
      multiple: false,
      defaultPath: this.defaultPath,
    });
    if (pathFromConfigFile !== null) {
      this.pathFieldVal = pathFromConfigFile + '\\';
    }
  }

  save() {
    if (this.configFilePath !== '') {
      writeTextFile(this.configFilePath, this.pathFieldVal)
    }
    var qt_ = 'QT:'
    var biosQt_ = 'BIOS QT:'
    var cr_lf = '\n';
    var emailContent = qt_+
                      this.mails[0].address+
                      cr_lf+biosQt_+
                      this.mails[1].address;
    writeTextFile(this.configFileMail, emailContent)
  }
}
