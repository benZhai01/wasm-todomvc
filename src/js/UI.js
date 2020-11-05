export class UI {

    constructor(model){
        this.model = model;
        this.init();
        this.bindEvent();
    }

    set filterType(v) {
        if(this.filter_type == v){
            return;
        }
        this.list_wrapper.dataset.selected = v;
        this.filter_type = v;
        for(let i = 0; i < this.filter_wrapper.children.length; i++){
            let node = this.filter_wrapper.children[i];
            node.dataset.active = false;
            if(node.dataset.type == v){
                node.dataset.active = true;
            }
        }
    }

    set completedAll(v) {
        if(v == this._completedAll){
            return;
        }
        this._completedAll = v;
        this.completedAll_wrapper.dataset.active = !!this._completedAll;
    }

    init() {
        this.wrapper = document.getElementById('todo_app');
        this.filter_wrapper = this.wrapper.querySelector('.filter_type');
        this.list_wrapper = this.wrapper.querySelector('.list');
        this.completedAll_wrapper = this.wrapper.querySelector('.toggle');
        this.footer_info_wrapper = this.wrapper.querySelector('.task_count');
        this.task_editor = this.wrapper.querySelector('.task_editor');
        this.filterType = 'all';
        this.items = [];
        this.model.onChanged = (e) => this.modelChanged(e);
    }

    bindEvent(){
        this.filter_wrapper.addEventListener('click', (e) => {
            if(!e.target || !e.target.dataset.type){
                return;
            }
            this.filterType = e.target.dataset.type;
        });
        this.wrapper.querySelector('.action').addEventListener('click', (e) => {
            let ids = this.items.filter(i => i.completed).map(i => i.id.toString());
            if(ids.length){
                this.model.delete(ids);
            }
        });
        this.completedAll_wrapper.addEventListener('click', () => {
            this.completedAll = this._completedAll === undefined ? true : !this._completedAll;
            let items = this.items.filter(i => i.completed != this._completedAll);
            items.forEach(i => i.completed = this._completedAll);
            if(items.length > 0){
                this.model.updateItems(items);
            }
        })
        this.list_wrapper.addEventListener('click', (e) => {
            if(!e.target) return;
            if(e.target.tagName == 'INPUT'){
                let id = e.target.parentElement.parentElement.dataset.id;
                let item = this.items.find(i => i.id == id);
                item.completed = e.target.checked;
                this.model.upsert(item);
            }else if(e.target.tagName == 'SPAN'){
                let id = e.target.parentElement.parentElement.dataset.id;
                this.model.delete(id);
            }
        });
        this.task_editor.addEventListener('keydown', (e)=> {
            if(e.keyCode == 13 && this.task_editor.value != ''){
                this.model.upsert({
                    title: this.task_editor.value,
                    completed: false
                });
                this.task_editor.value = '';
                e.preventDefault();
            }
        })
    }

    appendList(){
        this.items.forEach(i => this.list_wrapper.appendChild(this.createListItem(i)));
    }

    createListItem(item) {
        let div =document.createElement('div');
        div.className = 'list-item';
        div.dataset.completed = item.completed;
        div.dataset.id = item.id;
        div.innerHTML = `
        <div class="check-box">
            <input type='checkbox'/>
        </div>
        <label></label>
        <div class="clear_item">
            <span>×</span>
        </div>`;
        div.querySelector('input').checked = item.completed;
        div.querySelector('label').innerText = item.title;
        return div;
    }

    updateFooterInfo(){
        this.footer_info_wrapper.innerText = `${this.model.leftCount()} items left`;
    }

    modelChanged(args) {
        this.items = this.model.getItems();
        switch(args._type){
            case 'fill':
            case 'batch_delete':
            case 'batch_update':
                this.list_wrapper.innerHTML = '';
                this.appendList();
                break;
            case 'update':
                let id = args.detail.item.id;
                this.list_wrapper.querySelector(`[data-id="${id}"]`).dataset.completed = args.detail.item.completed;
                break;
            case 'delete':
                let node = this.list_wrapper.querySelector(`[data-id="${args.detail.item.id}"]`);
                this.list_wrapper.removeChild(node);
                break;
            case 'add': 
                this.list_wrapper.appendChild(this.createListItem(args.detail.item));
                this.completedAll = undefined
                break;
        }
        this.updateFooterInfo();
    }
}