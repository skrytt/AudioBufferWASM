import init, { Data } from '../pkg/audio_buffer_wasm.js';

async function run() {
    await init();

    let data = new Data();

    var button = document.getElementById('button');
    button.addEventListener("click", event => {
        console.log('Button clicked');
        data.generate_white_noise();
    });
}

run();
