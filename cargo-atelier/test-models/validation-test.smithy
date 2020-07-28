namespace org.example.smithy

structure MyStructure {
    known: String,
    wrongType: SomeOperation,
}

operation SomeOperation {
    input: SomeService
}

service SomeService {
    version: "1.0",
    operations: [MyStructure]
}