import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { ChipRenderModule } from './chip-render/chip-render.module';
import { ChipStackComponent } from './chip-stack.component';

export const chipStackImports = [
  ...([CommonModule, ChipRenderModule] as const),
];

@NgModule({
  imports: chipStackImports,
  declarations: [ChipStackComponent],
  exports: [ChipStackComponent],
})
export class ChipStackModule {}
