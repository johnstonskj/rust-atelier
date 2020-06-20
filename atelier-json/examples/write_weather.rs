use atelier_core::error::ErrorSource;
use atelier_core::io::write_model_to_string;
use atelier_core::model::builder::values::{ArrayBuilder, ObjectBuilder};
use atelier_core::model::builder::{
    Builder, ListBuilder, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder,
    ServiceBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::{Identifier, Model, ShapeID};
use atelier_json::io::JsonWriter;
use std::str::FromStr;

fn main() {
    let mut writer = JsonWriter::default();
    let model = make_weather_model();
    let output = write_model_to_string(&mut writer, &model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}

fn make_weather_model() -> Model {
    ModelBuilder::new("example.weather")
        .shape(
            ServiceBuilder::new("Weather")
                .doc_comment("Provides weather forecasts.")
                .paginated(Some("nextToken"), Some("nextToken"), None, Some("pageSize"))
                .version("2006-03-01")
                .resource("City")
                .operation("GetCurrentTime")
                .build(),
        )
        .shape(
            ResourceBuilder::new("City")
                .identifier("cityID", "CityID")
                .read("GetCity")
                .list("ListCities")
                .resource("Forecast")
                .build(),
        )
        .shape(
            ResourceBuilder::new("Forecast")
                .identifier("cityId", "CityId")
                .read("GetForecast")
                .build(),
        )
        .shape(
            SimpleShapeBuilder::string("CityId")
                .add_trait(TraitBuilder::pattern("^[A-Za-z0-9 ]+$").build())
                .build(),
        )
        .shape(
            OperationBuilder::new("GetCity")
                .readonly()
                .input("GetCityInput")
                .output("GetCityOutput")
                .error("NoSuchResource")
                .build(),
        )
        .shape(
            StructureBuilder::new("GetCityInput")
                .add_member(
                    MemberBuilder::new("cityID")
                        .required()
                        .refers_to("CityId")
                        .build(),
                )
                .build(),
        )
        .shape(
            StructureBuilder::new("GetCityOutput")
                .add_member(MemberBuilder::string("name").required().build())
                .add_member(
                    MemberBuilder::new("coordinates")
                        .required()
                        .refers_to("CityCoordinates")
                        .build(),
                )
                .build(),
        )
        .shape(
            StructureBuilder::new("CityCoordinates")
                .add_member(MemberBuilder::float("latitude").required().build())
                .add_member(MemberBuilder::float("longitude").required().build())
                .build(),
        )
        .shape(
            StructureBuilder::new("NoSuchResource")
                .error(ErrorSource::Client)
                .add_member(MemberBuilder::string("resourceType").required().build())
                .build(),
        )
        .shape(
            OperationBuilder::new("ListCities")
                .paginated(None, None, Some("items"), None)
                .readonly()
                .input("ListCitiesInput")
                .output("ListCitiesOutput")
                .build(),
        )
        .shape(
            StructureBuilder::new("ListCitiesInput")
                .string("nextToken")
                .integer("pageSize")
                .build(),
        )
        .shape(
            StructureBuilder::new("ListCitiesOutput")
                .string("nextToken")
                .add_member(
                    MemberBuilder::new("items")
                        .required()
                        .refers_to("CitySummaries")
                        .build(),
                )
                .build(),
        )
        .shape(ListBuilder::new("CitySummaries", "CitySummary").build())
        .shape(
            StructureBuilder::new("CitySummary")
                .add_trait(
                    TraitBuilder::references(
                        ArrayBuilder::default()
                            .push(
                                ObjectBuilder::default()
                                    .reference(
                                        Identifier::from_str("resource").unwrap().into(),
                                        ShapeID::from_str("City").unwrap(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
                )
                .add_member(
                    MemberBuilder::new("cityId")
                        .required()
                        .refers_to("CityId")
                        .build(),
                )
                .add_member(MemberBuilder::string("name").required().build())
                .build(),
        )
        .shape(
            OperationBuilder::new("GetCurrentTime")
                .readonly()
                .output("GetCurrentTimeOutput")
                .build(),
        )
        .shape(
            StructureBuilder::new("GetCurrentTimeOutput")
                .add_member(MemberBuilder::timestamp("time").required().build())
                .build(),
        )
        .shape(
            OperationBuilder::new("GetForecast")
                .readonly()
                .input("GetForecastInput")
                .output("GetForecastOutput")
                .build(),
        )
        .shape(
            StructureBuilder::new("GetForecastInput")
                .add_member(
                    MemberBuilder::new("cityId")
                        .required()
                        .refers_to("CityId")
                        .build(),
                )
                .build(),
        )
        .shape(
            StructureBuilder::new("GetForecastOutput")
                .float("chanceOfRain")
                .build(),
        )
        .build()
}
