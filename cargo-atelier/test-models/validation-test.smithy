namespace org.example.smithy

@unknownTrait
structure MyStructure {
    known: String,
    unknown: NotString,
    wrongType: SomeOperation,
}

operation SomeOperation {
    input: SomeService
}

service SomeService {
    operations: [MyStructure]
}