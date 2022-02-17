function insertChildAlphabetically(container, item, name) {
    for (var ii = 0; ii < container.children.length; ++ii) {
        if (container.children[ii].id > name) {
            container.insertBefore(item, container.children[ii]);
            return;
        }
    }

    container.append(item);
}

function tuneableToTag(tuneable) {
    if (tuneable.hasOwnProperty("Boolean")) {
        return "bool";
    }

    if (tuneable.hasOwnProperty("Int32") || tuneable.hasOwnProperty("Int64")) {
        return "int";
    }

    if (
        tuneable.hasOwnProperty("Float32") ||
        tuneable.hasOwnProperty("Float64")
    ) {
        return "float";
    }

    throw "Unknown variable type: " + tuneable;
}

function getInner(tuneable) {
    if (tuneable.hasOwnProperty("Boolean")) {
        return tuneable["Boolean"][0];
    }

    if (tuneable.hasOwnProperty("Int32")) {
        return tuneable["Int32"][0];
    }

    if (tuneable.hasOwnProperty("Float32")) {
        return tuneable["Float32"][0];
    }

    if (tuneable.hasOwnProperty("Int64")) {
        return tuneable["Int64"][0];
    }

    if (tuneable.hasOwnProperty("Float64")) {
        return tuneable["Float64"][0];
    }

    throw "Unknown variable type: " + tuneable;
}

function tuneableToValue(tuneable) {
    return getInner(tuneable).current;
}

function tuneableToWidgetConfig(type, tuneable, widget) {
    switch (type) {
        case "float":
        case "int": {
            let inner = getInner(tuneable);
            widget.min = inner.min;
            widget.max = inner.max;
            widget.default = inner.default;
            widget.value = inner.current;
            break;
        }

        case "bool": {
            let inner = tuneable["Bool"];
        }
    }
}
////////////////////////////////////////////////////////////////////////////////

class VarGroup {
    constructor(category) {
        this.name = category;
        this.variables = [];
        this.widget = undefined;
    }

    ////////////////////////////////////////////////////////////////////////////////

    addVariable(variable) {
        let widget = variable.createWidget();
        this.variables.push(variable);
        insertChildAlphabetically(this.body, widget, widget.id);
    }

    ////////////////////////////////////////////////////////////////////////////////

    filter(needle) {
        let anyVisible = false;
        this.variables.forEach((value, key) => {
            if (value.fullName.toLowerCase().includes(needle)) {
                value.show();
                anyVisible = true;
            } else {
                value.hide();
            }
        });

        this.widget.hidden = !anyVisible;
    }

    ////////////////////////////////////////////////////////////////////////////////

    createTable() {
        var html = document.getElementById("placeholder-table");

        let newNode = document.importNode(html.content, true);
        newNode.id = this.name + "-group";
        newNode.querySelector(".card-header").innerText = this.name;
        newNode.querySelector("div").id = this.name;

        this.body = newNode.querySelector(".table-body");

        let container = document.getElementById("data-container");
        insertChildAlphabetically(container, newNode, this.name);

        this.widget = document.getElementById(this.name);
    }
}

////////////////////////////////////////////////////////////////////////////////

class Var {
    constructor(category, name, tuneable) {
        this.category = category;
        this.fullName = `${category}.${name}`;
        this.name = name;
        this.tuneable = tuneable;
        this.type = tuneableToTag(tuneable);
        this.value = tuneableToValue(tuneable);
        this.topWidget = null;
    }

    ////////////////////////////////////////////////////////////////////////////////

    setValue(tuneable) {
        tuneableToWidgetConfig(this.type, tuneable, this.realWidget);
        this.tuneable = tuneable;
        this.value = tuneableToValue(tuneable);
        switch (this.type) {
            case "float":
            case "int":
                this.realWidget.value = this.value;
                break;
            case "bool":
                this.realWidget.checked = this.value;
                break;
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    show() {
        this.topWidget.hidden = false;
    }

    ////////////////////////////////////////////////////////////////////////////////

    hide() {
        this.topWidget.hidden = true;
    }

    ////////////////////////////////////////////////////////////////////////////////

    createWidget() {
        var html = document.getElementById("placeholder-row");

        let newNode = document.importNode(html.content, true);
        let row = newNode.querySelector("tr");
        row.id = this.name;

        let label = row.children[0];
        label.innerText = this.name;

        let widget = document.createElement("input");
        switch (this.type) {
            case "int": // int
                widget.type = "range";
                widget.value = this.value;
                tuneableToWidgetConfig(this.type, this.tuneable, widget);
                break;
            case "float":
                widget.type = "range";
                tuneableToWidgetConfig(this.type, this.tuneable, widget);
                break;
            case "bool":
                widget.type = "checkbox";
                widget.checked = this.value;
                break;
        }

        console.log(widget, this.type);
        widget.onchange = this.onchange.bind(this);
        row.children[1].appendChild(widget);
        this.lock = row.children[2].children[0];
        this.realWidget = widget;
        this.topWidget = row;
        this.topWidget.classList.add("table-success");

        return row;
    }

    ////////////////////////////////////////////////////////////////////////////////

    onchange(e) {
        switch (this.type) {
            case "int":
                this.value = e.target.value;
                getInner(this.tuneable).current = parseInt(this.value);
                break;
            case "float":
                this.value = e.target.value;
                getInner(this.tuneable).current = parseFloat(this.value);
                break;
            case "bool":
                this.value = e.target.checked;
                getInner(this.tuneable).current = this.value;
                break;
        }

        this.topWidget.classList.remove("table-success");
        this.topWidget.classList.add("table-warning");
        window.tuna.set(this.category, this.name, this.tuneable);
    }
}

class Vars {
    constructor() {
        this.groups = new Map();
        this.vars = new Map();
        document.getElementById("-searchbox").oninput = this.filter.bind(this);
    }

    ////////////////////////////////////////////////////////////////////////////////

    filter(eve) {
        let needle = eve.target.value.toLowerCase();
        this.groups.forEach((group) => group.filter(needle));
    }

    reset() {}

    disconnect() {}

    ////////////////////////////////////////////////////////////////////////////////

    addData(msg) {
        if (msg.hasOwnProperty("Tuneables")) {
            let payload = msg.Tuneables[0];

            for (let [category, children] of Object.entries(payload)) {
                const variables = Object.entries(children);

                for (let [name, details] of variables) {
                    this.createVariable(category, name, details);
                }
            }
        } else if (msg.hasOwnProperty("Delta")) {
            const [category, name, tuneable] = msg["Delta"];
            this.updateVariable(category, name, tuneable);
        } else if (msg.hasOwnProperty("Ok")) {
            const [[category, name]] = msg["Ok"];
            this.ok(category, name);
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    createVariable(category, name, value) {
        let fullName = `${category}.${name}`;
        if (this.vars.has(fullName)) {
            this.vars.get(fullName).setValue(value);
            return;
        }

        let _var = new Var(category, name, value);
        this.vars.set(fullName, _var);

        if (!this.groups.has(category)) {
            let varGroup = new VarGroup(category);
            varGroup.createTable();
            this.groups.set(category, varGroup);
        }

        this.groups.get(category).addVariable(_var);
    }

    ////////////////////////////////////////////////////////////////////////////////

    updateVariable(category, name, value) {
        let fullName = `${category}.${name}`;
        let item = this.vars.get(fullName);
        item.setValue(value);

        item.topWidget.classList.add("table-success");
        item.topWidget.classList.remove("table-warning");
    }

    ok(category, name) {
        let fullName = `${category}.${name}`;
        let item = this.vars.get(fullName);

        item.topWidget.classList.add("table-success");
        item.topWidget.classList.remove("table-warning");
    }
}
