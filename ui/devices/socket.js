import Toucher from './../toucher';

class HeimaSocket extends HTMLElement {
    connectedCallback() {
        this.device = store.get_device(this.getAttribute("id"));

        // create shadow dom
        this.attachShadow({ mode: 'open' });

        // create and add the icon and toucher
        this.toucher = new Toucher();
        this.toucher.innerHTML = `<live-icon-socket power="${this.device.status["Socket"].power}"></live-icon-socket>`;
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
        var icon = this.toucher.querySelector("live-icon-socket");

        // set the new state
        if (icon.getAttribute("power") === "0") {
            this.device.updateStatus("Socket", { power: 1.0 });
        }
        else {
            this.device.updateStatus("Socket", { power: 0 });
        }
    }

    update() {
        // get the icon
        var icon = this.toucher.querySelector("live-icon-socket");

        // only update icon if the value really has changed to keep animation running
        if (parseFloat(icon.getAttribute("power")) !== this.device.status["Socket"].power) {
            this.toucher.querySelector("live-icon-socket").setAttribute("power", this.device.status["Socket"].power);
        }
    }
}
customElements.define("heima-socket", HeimaSocket);