namespace org.example.smithy

@ThisIsNotAGoodName
structure thisIsMyStructure {
    lower: String,
    Upper: String,
    someJSONThing: someUnknownShape,
    OK: Boolean
}

string someUnknownShape

@trait
structure ThisIsNotAGoodName {}