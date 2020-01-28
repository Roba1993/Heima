class HeimaRoom extends HTMLElement {
    connectedCallback() {
        this.name = this.getAttribute("name");
        this.attachShadow({ mode: 'open' });

        var devices = ``;
        store.get_room_devices(this.name).forEach(d => {
            devices += `<heima-device id="${d.id}"></heima-device>`;
        });

        var html = `
            <h1 class="name">${this.name}</h1>
            
            <div class="devices">
                ${devices}
            </div>
        `;

        var style = `
            .devices {
                display: flex;
                flex-wrap: wrap;
                flex-direction: row;
            }

            .name {
                font-family: Sans-serif;
            }
        `;

        this.shadowRoot.innerHTML = html + `<style>${style}</style>`;
    }
}
customElements.define("heima-room", HeimaRoom);