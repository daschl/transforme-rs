import * as popscale from "transforme-rs";


var doc = {
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


console.log(popscale.transform(doc, new_schema));
