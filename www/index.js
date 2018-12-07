import * as popscale from "transforme-rs";


var old_doc = {
    "firstName": "Michael"
};

var new_schema = {
    "firstName": {
        "type": "string"
    },
    "lastName": {
        "type": "string"
    }
}


console.log(popscale.transform(old_doc, new_schema));


var new_doc = {
    "firstName": "Michael",
    "lastName": "Nitschinger"
};

var old_schema = {
    "firstName": {
        "type": "string"
    }
}


console.log(popscale.transform(new_doc, old_schema));