/*!
This example shows how to use the builder interface to create a copy of the weather sample
model from the Smithy [Quick Start](https://awslabs.github.io/smithy/quickstart.html#complete-example)
document.
*/

use atelier_core::builder::values::{ArrayBuilder, ObjectBuilder};
use atelier_core::builder::{
    ListBuilder, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder,
    SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::error::ErrorSource;
use atelier_core::io::debug::DebugWriter;
use atelier_core::io::write_model_to_string;
use atelier_core::model::Model;
use atelier_core::Version;

fn main() {
    let mut writer = DebugWriter::default();
    let model = make_weather_model();
    let output = write_model_to_string(&mut writer, &model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}

fn make_weather_model() -> Model {
    ModelBuilder::new(Version::V10)
        .default_namespace("example.weather")
        .shape(
            ServiceBuilder::new("Weather", "2006-03-01")
                .documentation("Provides weather forecasts.")
                .paginated(Some("nextToken"), Some("nextToken"), None, Some("pageSize"))
                .resource("City")
                .operation("GetCurrentTime")
                .into(),
        )
        .shape(
            ResourceBuilder::new("City")
                .identifier("cityID", "CityID")
                .read("GetCity")
                .list("ListCities")
                .resource("Forecast")
                .into(),
        )
        .shape(
            ResourceBuilder::new("Forecast")
                .identifier("cityId", "CityId")
                .read("GetForecast")
                .into(),
        )
        .shape(
            SimpleShapeBuilder::string("CityId")
                .apply_trait(TraitBuilder::pattern("^[A-Za-z0-9 ]+$").into())
                .into(),
        )
        .shape(
            OperationBuilder::new("GetCity")
                .readonly()
                .input("GetCityInput")
                .output("GetCityOutput")
                .error("NoSuchResource")
                .into(),
        )
        .shape(
            StructureBuilder::new("GetCityInput")
                .add_member(MemberBuilder::new("cityID", "CityId").required().into())
                .into(),
        )
        .shape(
            StructureBuilder::new("GetCityOutput")
                .add_member(MemberBuilder::string("name").required().into())
                .add_member(
                    MemberBuilder::new("coordinates", "CityCoordinates")
                        .required()
                        .into(),
                )
                .into(),
        )
        .shape(
            StructureBuilder::new("CityCoordinates")
                .add_member(MemberBuilder::float("latitude").required().into())
                .add_member(MemberBuilder::float("longitude").required().into())
                .into(),
        )
        .shape(
            StructureBuilder::new("NoSuchResource")
                .error(ErrorSource::Client)
                .add_member(MemberBuilder::string("resourceType").required().into())
                .into(),
        )
        .shape(
            OperationBuilder::new("ListCities")
                .paginated(None, None, Some("items"), None)
                .readonly()
                .input("ListCitiesInput")
                .output("ListCitiesOutput")
                .into(),
        )
        .shape(
            StructureBuilder::new("ListCitiesInput")
                .string("nextToken")
                .integer("pageSize")
                .into(),
        )
        .shape(
            StructureBuilder::new("ListCitiesOutput")
                .string("nextToken")
                .add_member(
                    MemberBuilder::new("items", "CitySummaries")
                        .required()
                        .into(),
                )
                .into(),
        )
        .shape(ListBuilder::new("CitySummaries", "CitySummary").into())
        .shape(
            StructureBuilder::new("CitySummary")
                .apply_trait(
                    TraitBuilder::references(
                        ArrayBuilder::default()
                            .push(
                                ObjectBuilder::default()
                                    .reference("resource", "City")
                                    .into(),
                            )
                            .into(),
                    )
                    .into(),
                )
                .add_member(MemberBuilder::new("cityId", "CityId").required().into())
                .add_member(MemberBuilder::string("name").required().into())
                .into(),
        )
        .shape(
            OperationBuilder::new("GetCurrentTime")
                .readonly()
                .output("GetCurrentTimeOutput")
                .into(),
        )
        .shape(
            StructureBuilder::new("GetCurrentTimeOutput")
                .add_member(MemberBuilder::timestamp("time").required().into())
                .into(),
        )
        .shape(
            OperationBuilder::new("GetForecast")
                .readonly()
                .input("GetForecastInput")
                .output("GetForecastOutput")
                .into(),
        )
        .shape(
            StructureBuilder::new("GetForecastInput")
                .add_member(MemberBuilder::new("cityId", "CityId").required().into())
                .into(),
        )
        .shape(
            StructureBuilder::new("GetForecastOutput")
                .float("chanceOfRain")
                .into(),
        )
        .into()
}
