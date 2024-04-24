import { ApplicationConfig } from "@angular/core";
import { provideRouter } from "@angular/router";

import { routes } from "./app.routes";
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import {MatSlideToggleModule} from '@angular/material/slide-toggle';

export const appConfig: ApplicationConfig = {
  providers: [provideRouter(routes), provideAnimationsAsync()],
};
