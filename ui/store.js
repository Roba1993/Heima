/**
 * Store Parent Class to implement all stores from.
 */
class Store {
    constructor() {
        this.listener = [];

        return new Proxy(this, {
            set: (object, key, value, proxy) => {
                object[key] = value;

                this.sendListener(key);

                return true;
            }
        });
    }

    sendListener(key) {
        this.listener.forEach(l => {
            if (l.vars === null || l.vars === undefined || l.vars === key || l.vars.indexOf(key) >= 0) {
                l.func(this);
            }
        });
    }

    addListener(identifier, vars, func) {
        let listener = {
            id: identifier,
            vars: vars,
            func: func
        };

        let index = this.listener.indexOf(identifier);

        if (index >= 0) {
            this.listener[index] = listener;
        }
        else {
            this.listener.push(listener);
        }
    }

    removeListener(ident) {
        this.listener = this.listener.filter(o => {
            return o.id !== ident;
        });
    }
}


/////////// Example Store Implementation /////////////

class Device extends Store {
    constructor(id, name, rooms, status) {
        super();
        this.id = id;
        this.name = name;
        this.rooms = rooms;
        this.status = status;
    }

    updateStatus(device, values) {
        Object.keys(values).forEach(k => {
            this.status[device][k] = values[k];
        })


        this.status = this.status;
    }
}

var devices = [];
devices.push(new Device("0", "Light", ["Favorites", "Kitchen"], { DimLight: { power: 1.0 }, Meter: { power: 12.5, max: 30 } }));
devices.push(new Device("1", "Socket", ["Favorites", "Bath"], { Socket: { power: 0.0 }, Meter: { power: 5.0, max: 7.0 } }));
devices.push(new Device("2", "Blend", ["Favorites", "Guestroom"], { Blind: { open: 0.5, angle: 0.0 } }));


class GlobalStore extends Store {
    constructor() {
        super();
        this.url = "127.0.0.1:8888";
        this.devices = devices;
    }

    get_rooms() {
        var rooms = [];

        // loop each device
        this.devices.forEach(d => {
            // loop each room in device
            d.rooms.forEach(r => {
                // check if the room aleady exist
                if (!rooms.includes(r)) {
                    // add if not
                    rooms.push(r);
                }
            });
        });

        return rooms;
    }

    get_room_devices(room) {
        return this.devices.filter(d => {
            return d.rooms.includes(room);
        });
    }

    get_device(id) {
        return this.devices.find(d => {
            return d.id === id;
        });
    }
}

window.store = new GlobalStore();

window.update_test = function () {
    store.devices[0].name = "Light new";
    store.devices = store.devices;
}


function animateMeter() {
    store.devices.forEach(d => {
        if (d.status["Meter"] !== undefined) {
            var v = d.status["Meter"].power + ((Math.random() * 2) - 1);
            d.updateStatus("Meter", { power: v })
        }
    })


    window.setTimeout(animateMeter, 250);
}
animateMeter();
