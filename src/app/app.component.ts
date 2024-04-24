import { Component} from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { MatTabsModule } from '@angular/material/tabs'
import { UploadComponent } from './upload/upload.component';
import { UserSettingComponent } from './user-setting/user-setting.component';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, MatTabsModule, UploadComponent, UserSettingComponent ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})

export class AppComponent {
  
  
}
