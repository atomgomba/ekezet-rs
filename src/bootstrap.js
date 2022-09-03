import init, { run_text_anim, run_fade_in_anim } from './lib.js';

await init();
window.anim_handle = run_text_anim();
window.intro_handle = run_fade_in_anim();
