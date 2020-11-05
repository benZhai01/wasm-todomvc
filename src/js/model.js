export class Model {
    constructor(wasm) {
        this.onChanged = undefined;
        this.items = undefined;
    }

    setWasm(wasm) {
        this.wasm = wasm;
    }

    fill(){
        this.wasm.run("todo_data");
    }

    delete(id){
        if(Array.isArray(id)){
            this.wasm.delete_items(id);
        }else{
            this.wasm.delete_item(id);
        }
    }

    upsert(item){
        if(item.id){
            this.wasm.update_item(item);
        }else{
            item.id = Date.now().toString();
            this.wasm.add_item(item);
        }
    }

    updateItems(items) {
        this.wasm.update_items(items);
    }

    getItems(){
        return this.wasm.get_items();
    }

    leftCount() {
        return this.getItems().filter(item => !item.completed).length;
    }
}