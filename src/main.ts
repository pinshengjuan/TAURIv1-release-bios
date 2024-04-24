import { bootstrapApplication } from "@angular/platform-browser";
import { appConfig } from "./app/app.config";
import { AppComponent } from "./app/app.component";
import { appConfigDir } from '@tauri-apps/api/path';
import { writeTextFile, createDir, exists  } from '@tauri-apps/api/fs';

bootstrapApplication(AppComponent, appConfig).catch((err) =>
  console.error(err),
);

appConfigDir().then((configDir) => {
  createDir(configDir, { recursive: true }).then(()=>{
    var defaultPath = configDir+'default.path';
    var emailList = configDir+'email.list';
    exists(defaultPath).then((sts: boolean)=>{
      if(sts === false) {
        writeTextFile(defaultPath, '');
      }
    });
    exists(emailList).then((sts: boolean)=>{
      if(sts === false) {
        writeTextFile(emailList, '');
      }
    });
  });
});