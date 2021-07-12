mod common;
use common::parse_and_write_json;

#[test]
fn model_to_json_ex_17_2_1() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#MyString": {
            "type": "string"
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_2_3() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#MyString": {
            "type": "string",
            "traits": {
                "smithy.api#documentation": "My documentation string"
            }
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_3() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#Blob": {
            "type": "blob"
        },
        "smithy.example#Boolean": {
            "type": "boolean"
        },
        "smithy.example#Document": {
            "type": "document"
        },
        "smithy.example#String": {
            "type": "string"
        },
        "smithy.example#Byte": {
            "type": "byte"
        },
        "smithy.example#Short": {
            "type": "short"
        },
        "smithy.example#Integer": {
            "type": "integer"
        },
        "smithy.example#Long": {
            "type": "long"
        },
        "smithy.example#Float": {
            "type": "float"
        },
        "smithy.example#Double": {
            "type": "double"
        },
        "smithy.example#BigInteger": {
            "type": "bigInteger"
        },
        "smithy.example#BigDecimal": {
            "type": "bigDecimal"
        },
        "smithy.example#Timestamp": {
            "type": "timestamp"
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_4_1() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#MyList": {
            "type": "list",
            "member": {
                "target": "smithy.api#String"
            }
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_4_2() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#MySet": {
            "type": "set",
            "member": {
                "target": "smithy.api#String"
            }
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_5() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#MyList": {
            "type": "list",
            "member": {
                "target": "smithy.api#String",
                "traits": {
                    "smithy.api#documentation": "Member documentation"
                }
            }
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_6() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#Service": {
            "type": "service",
            "version": "2017-02-11",
            "operations": [
                {
                    "target": "smithy.example#Operation"
                }
            ]
        },
        "smithy.example#Operation": {
            "type": "operation"
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_7() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#IntegerMap": {
            "type": "map",
            "key": {
                "target": "smithy.api#String"
            },
            "value": {
                "target": "smithy.api#Integer"
            }
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_8_1() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#MyStructure": {
            "type": "structure",
            "members": {
                "stringMember": {
                    "target": "smithy.api#String",
                    "traits": {
                        "smithy.api#required": {}
                    }
                },
                "numberMember": {
                    "target": "smithy.api#Integer"
                }
            }
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_8_2() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#MyUnion": {
            "type": "union",
            "members": {
                "a": {
                    "target": "smithy.api#String"
                },
                "b": {
                    "target": "smithy.api#Integer"
                }
            }
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_9() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#MyService": {
            "type": "service",
            "version": "2017-02-11",
            "operations": [
                {
                    "target": "smithy.example#GetServerTime"
                }
            ],
            "resources": [
                {
                    "target": "smithy.example#SomeResource"
                }
            ],
            "traits": {
                "smithy.api#documentation": "Documentation for the service"
            },
            "rename": {
                "smithy.example#Widget": "SmithyWidget",
                "foo.example#Widget": "FooWidget"
            }
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_10() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#Thing": {
            "type": "resource",
            "identifiers": {
                "forecastId": {
                    "target": "smithy.api#String"
                }
            },
            "create": {
                "target": "smithy.example#CreateThing"
            },
            "read": {
                "target": "smithy.example#GetThing"
            },
            "update": {
                "target": "smithy.example#Updatething"
            },
            "delete": {
                "target": "smithy.example#DeleteThing"
            },
            "list": {
                "target": "smithy.example#ListThings"
            },
            "operations": [
                {
                    "target": "smithy.example#SomeInstanceOperation"
                }
            ],
            "collectionOperations": [
                {
                    "target": "smithy.example#SomeCollectionOperation"
                }
            ],
            "resources": [
                {
                    "target": "smithy.example#SomeResource"
                }
            ]
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_11() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#MyOperation": {
            "type": "operation",
            "input": {
                "target": "smithy.example#MyOperationInput"
            },
            "output": {
                "target": "smithy.example#MyOperationOutput"
            },
            "errors": [
                {
                    "target": "smithy.example#BadRequestError"
                },
                {
                    "target": "smithy.example#NotFoundError"
                }
            ]
        },
        "smithy.example#MyOperationInput": {
            "type": "structure"
        },
        "smithy.example#MyOperationOutput": {
            "type": "structure"
        },
        "smithy.example#BadRequestError": {
            "type": "structure",
            "traits": {
                "smithy.api#error": "client"
            }
        },
        "smithy.example#NotFoundError": {
            "type": "structure",
            "traits": {
                "smithy.api#error": "client"
            }
        }
    }
}"#,
    )
}

#[test]
fn model_to_json_ex_17_12() {
    parse_and_write_json(
        r#"{
    "smithy": "1.0",
    "shapes": {
        "smithy.example#Struct": {
            "type": "structure",
            "members": {
                "foo": {
                    "target": "smithy.api#String"
                }
            }
        },
        "smithy.example#Struct$foo": {
            "type": "apply",
            "traits": {
                "smithy.api#documentation": "My documentation string"
            }
        }
    }
}"#,
    )
}
