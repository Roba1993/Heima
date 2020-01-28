class HeimaDevice extends HTMLElement {
    connectedCallback() {
        this.device = store.get_device(this.getAttribute("id"));
        this.attachShadow({ mode: 'open' });

        var html = `
        <div class="name">${this.device.name}</div>
        <heima-slider width="250px" height="250px">
            ${this.genDevices()}
        </heima-slider>
        
        <style>
            :host {
                margin: 0px 10px;
                background-color: white;
                height: 300px;
            }

            .name {
                text-align: center;
                margin: 5px;
                font-size: 20px;
                font-weight: bold;
                font-family: Sans-serif;
            }
        </style>
        `;

        this.shadowRoot.innerHTML = html;
    }

    genDevices() {
        var html = ``;

        Object.keys(this.device.status).forEach(k => {
            switch (k) {
                case "Light":
                    html += `<span class="slide"><heima-light id="${this.device.id}" ></heima-light></span>`;
                    break;
                case "DimLight":
                    html += `<span class="slide"><heima-dim-light id="${this.device.id}" ></heima-dim-light></span>`;
                    break;
                case "Socket":
                    html += `<span class="slide"><heima-socket id="${this.device.id}" ></heima-socket></span>`;
                    break;
                case "Blind":
                    html += `<span class="slide"><heima-blind id="${this.device.id}" ></heima-blind></span>`;
                    break;
                case "Meter":
                    html += `<span class="slide"><heima-meter id="${this.device.id}" ></heima-meter></span>`;
                    break;
            }
        })

        return html;
    }
}
customElements.define("heima-device", HeimaDevice);