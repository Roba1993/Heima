import Toucher from './../toucher';

class HeimaLight extends HTMLElement {
    connectedCallback() {
        this.device = store.get_device(this.getAttribute("id"));

        // create shadow dom
        this.attachShadow({ mode: 'open' });

        // create and add the icon and toucher
        this.toucher = new Toucher();
        this.toucher.innerHTML = `<live-icon-light power="${this.device.status["Light"].power}"></live-icon-light>`;
        this.shadowRoot.appendChild(this.toucher);

        // set the toucher actions
        this.toucher.onShortClick = this.toggleLight.bind(this);
        this.toucher.onMediumClick = this.toggleLight.bind(this);
        this.toucher.onLongClick = this.toggleLight.bind(this);

        this.device.addListener(this, null, this.update.bind(this));
    }

    disconnectedCallback() {
        this.device.removeListener(this);
    }

    toggleLight() {
        // get the icon
        var icon = this.toucher.querySelector("live-icon-light");

        // set the new state
        if (icon.getAttribute("power") === "0") {
            this.device.updateStatus("Light", { power: 1.0 });
        }
        else {
            this.device.updateStatus("Light", { power: 0 });
        }
    }

    update() {
        this.toucher.querySelector("live-icon-light").setAttribute("power", this.device.status["Light"].power);
    }
}
customElements.define("heima-light", HeimaLight);