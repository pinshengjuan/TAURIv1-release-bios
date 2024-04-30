import { Component, NgZone } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { FormsModule} from '@angular/forms';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faDiagramProject, faDatabase, faFileInvoice, faPaperPlane, faClipboard, faPowerOff, faUpload, faEllipsis} from '@fortawesome/free-solid-svg-icons';
import { MatSlideToggleModule } from '@angular/material/slide-toggle';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { open, message } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { appConfigDir } from '@tauri-apps/api/path';
import { readTextFile, exists } from '@tauri-apps/api/fs';
import { listen } from '@tauri-apps/api/event';
import { environment } from '../environments/environment'; //environment variables, add by self

var debug = environment.production ? 0 : 1;

@Component({
  selector: 'app-upload',
  standalone: true,
  imports: [CommonModule, RouterOutlet, FontAwesomeModule, MatSlideToggleModule, FormsModule, MatProgressSpinnerModule],
  templateUrl: './upload.component.html',
  styleUrl: './upload.component.css'
})


export class UploadComponent {
  greetingMessage = '';
  status = 'Release';
  faUpload = faUpload;
  uploadIcon = true;
  projectPath = '';
  supervisorMail = ';'

  faFileInvoice = faFileInvoice;
  faEllipsisH = faEllipsis;
  faDatabase = faDatabase;

  allOptions: any[] = [
    { icon: faDiagramProject, action: 'Production', id: 'isProduction', check: false },
    { icon: faPaperPlane, action: 'Generate Email', id: 'isEmail', check: true },
    { icon: faClipboard, action: 'Copy History Content to Clipboard', id: 'isCopy', check: false },
    { icon: faPowerOff, action: 'Close When Finished', id: 'isClose', check: true },
  ];
  
  paths: any[] = [
    { icon: faFileInvoice, action: 'History', inputId: 'historyId', ellipsisId: 'ellipsisHistoryId', thePath:''},
    { icon: faFileInvoice, action: 'Rom Server', inputId: 'romId', ellipsisId: 'ellipsisRomId', thePath:''},
  ];

  _dir = appConfigDir().then((configDir) => {
    const defaultPath = configDir+'default.path';
    this.supervisorMail = configDir+'email.list';
    exists(defaultPath).then((sts: boolean)=>{
      if (sts === true) {
        readTextFile(defaultPath).then((folder)=>{
          if (folder.trim()) {
            invoke<string>('check_folder', { folder }).then( (text) => {
              if (text === 'true') {
                this.projectPath = folder;
              }
            });
          }
        });
      }
    });
  });

  constructor(private zone: NgZone) { }

  _drop = listen('tauri://file-drop',  (event) => {
    const input = ((event.payload as string)[0] as string).replace("/", "\\");
    const historyPattern = new RegExp(/History[a-zA-Z\d\s\.\-\_]+txt/);
    if (input.slice(input.lastIndexOf("\\")).match(historyPattern)) {
      this.zone.run(() => { //use this to prevent
        this.paths[0].thePath = input;
        this.checkBiosFile(input, 'historyId');
      });
    }
  });

  async handleInput(inputId: string) {
    if (inputId === 'historyId') {
      if (debug) {
        this.projectPath = environment.localPath;
      }
      if (this.paths[0].thePath) {
        this.projectPath = this.paths[0].thePath
      }
      const txtFile = await open({
        directory: false,
        multiple: false,
        filters: [{
          name: 'History',
          extensions: ['txt']
        }],
        defaultPath: this.projectPath
      });
      if (txtFile) {
        this.paths[0].thePath = txtFile;
      }
      await this.checkBiosFile(txtFile, 'historyId');
    } else {
      var serverPath = environment.serverPath;
      if (this.paths[1].thePath) {
        serverPath = this.paths[1].thePath
      }

      const server = await open({
        directory: true,
        multiple: false,
        defaultPath: serverPath,
      });
      if (server !== null) {
        this.paths[1].thePath = server + '\\';
      }
    }
  }

  async checkBiosFile(txtFile: any, inputId: string) {
    if (inputId === 'historyId') {
      this.uploadIcon = false;
      this.status = 'Checking BIOS ROM file...'
      let txt: string = txtFile;
      if (txt!==null) {
        invoke<string>('is_bios_file_exists', { txt }).catch( async (error) => {
          await message(error, { title: ' '});
        });
      }
    }
    this.uploadIcon = true;
    this.status = 'Release'
  }

  checkDependency() {
    if (this.allOptions[0].check) {
      // this.allOptions[1].check = true;
    }
  }

  async summit() {
    if (this.paths[0].thePath && this.paths[1].thePath) {
      let txt: string = this.paths[0].thePath;
      let server: string = this.paths[1].thePath;
      let supervisorMail = this.supervisorMail;

      this.uploadIcon = false;
      this.status = 'Checking BIOS ROM exists...';
      await this.checkBiosFile(this.paths[0].thePath, 'historyId');

      this.status = 'Copying History.txt to server...';
      await invoke<string>('copy_history_file_to_server', { txt, server });

      this.status = 'Copying BIOS ROM to server...';
      await invoke<string>('copy_bios_file_to_server', { txt, server });

      if (this.allOptions[0].check) { //production
        this.status = 'Check Production folder exists...';
        await invoke<string>('is_production_folder_exists', { server });
 
        await invoke<string>('is_version_folder_exists', { txt, server });
        
        this.status = 'Copying BIOS ROM to Production folder...';
        await invoke<string>('copy_bios_file_to_production', { txt, server });
        
        this.status = 'Creating checksum file...';
        await invoke<string>('make_checksum_file', { txt, server });
        
        this.status = 'Packing ROM in Production folder...';
        await invoke<string>('pack_rom', { txt, server });
      }

      if (this.allOptions[1].check) { //outlook
        this.status = 'Open Outlook...';
        let production: boolean = this.allOptions[0].check;

        await invoke<string>('open_outlook', { txt, server, production, supervisorMail});
      }

      if (this.allOptions[2].check) { //clipboard
        this.status = 'Copying email content to clipboard...';
        let production: boolean = this.allOptions[0].check;

        await invoke<string>('set_clipboard', { txt, server, production});
      }
    }

    this.uploadIcon = true;
    this.status = 'Release';
    if (this.allOptions[3].check) {
      appWindow.close();
    }
  }
}
