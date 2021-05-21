$version: "1.0"

namespace earth.kingdom.animals.mammals.dogs

use earth.kingdom.animals#Class

structure Dog {
    @required
    @documentation("animal classification (from earth.kingdom.animals#Class)")
    class: Class,

    @required
    @documentation("the breed of this dog (from earth.kingdom.animals.mammals.dogs#Breed)")
    breed: Breed,

    @required
    @documentation("the color of this dog (from earth.kingdom.animals.mammals.dogs#Color)")
    color: Color,
}
