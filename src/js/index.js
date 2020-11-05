import { Model } from './model';
import { UI } from './UI';
const rust = import('../../pkg');

const model = new Model();
window.model_updated = (arg) => {
    model.onChanged(arg)
}

rust.then(wasm => {
    model.setWasm(wasm);
    let app = new UI(model);
    model.fill()
}).catch(console.error)