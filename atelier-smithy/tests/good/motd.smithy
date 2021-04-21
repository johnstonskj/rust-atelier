$version: "1.0"

namespace example.motd

@readonly
operation GetMessage {
   input: GetMessageInput
   output: GetMessageInput
   errors: [
      BadDateValue
   ]
}

@pattern("^\\d\\d\\d\\d\\-\\d\\d\\-\\d\\d$")
string Date

structure GetMessageOutput {
   @required
   message: String
}

resource Message {
   identifiers: {
      date: Date
   }
   read: GetMessage
}

@documentation("Provides a Message of the day.")
service MessageOfTheDay {
   version: "2020-06-21"
   resources: [
      Message
   ]
}

structure GetMessageInput {
   date: example.motd#Date
}

@error("client")
structure BadDateValue {
   @required
   errorMessage: String
}
