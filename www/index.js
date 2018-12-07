import * as popscale from "transforme-rs";


var doc = {"hello": "world"};
var old_schema = {
    "hello": "string"
}
var new_schema = {
    "hello": "array"
}
console.log(popscale.transform(doc, old_schema, new_schema));
