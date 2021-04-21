use atelier_core::builder::values::{ArrayBuilder, ObjectBuilder};
use atelier_core::builder::{
    traits, ListBuilder, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder,
    ServiceBuilder, ShapeTraits, SimpleShapeBuilder, StructureBuilder,
};
use atelier_core::error::ErrorSource;
use atelier_core::io::write_model_to_string;
use atelier_core::model::{Model, NamespaceID};
use atelier_core::Version;
use atelier_smithy::SmithyWriter;

fn make_weather_model() -> Model {
    ModelBuilder::new(Version::V10, "example.weather")
        .service(
            ServiceBuilder::new("Weather", "2006-03-01")
                .documentation("Provides weather forecasts.")
                .paginated(Some("nextToken"), Some("nextToken"), None, Some("pageSize"))
                .resource("City")
                .operation("GetCurrentTime")
                .into(),
        )
        .resource(
            ResourceBuilder::new("City")
                .identifier("cityId", "CityId")
                .read("GetCity")
                .list("ListCities")
                .resource("Forecast")
                .into(),
        )
        .resource(
            ResourceBuilder::new("Forecast")
                .identifier("cityId", "CityId")
                .read("GetForecast")
                .into(),
        )
        .simple_shape(
            SimpleShapeBuilder::string("CityId")
                .apply_trait(traits::pattern("^[A-Za-z0-9 ]+$"))
                .into(),
        )
        .operation(
            OperationBuilder::new("GetCity")
                .readonly()
                .input("GetCityInput")
                .output("GetCityOutput")
                .error("NoSuchResource")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetCityInput")
                .add_member(MemberBuilder::new("cityID", "CityId").required().into())
                .into(),
        )
        .structure(
            StructureBuilder::new("GetCityOutput")
                .add_member(MemberBuilder::string("name").required().into())
                .add_member(
                    MemberBuilder::new("coordinates", "CityCoordinates")
                        .required()
                        .into(),
                )
                .into(),
        )
        .structure(
            StructureBuilder::new("CityCoordinates")
                .add_member(MemberBuilder::float("latitude").required().into())
                .add_member(MemberBuilder::float("longitude").required().into())
                .into(),
        )
        .structure(
            StructureBuilder::new("NoSuchResource")
                .error_source(ErrorSource::Client)
                .add_member(MemberBuilder::string("resourceType").required().into())
                .into(),
        )
        .operation(
            OperationBuilder::new("ListCities")
                .paginated(None, None, Some("items"), None)
                .readonly()
                .input("ListCitiesInput")
                .output("ListCitiesOutput")
                .into(),
        )
        .structure(
            StructureBuilder::new("ListCitiesInput")
                .string("nextToken")
                .integer("pageSize")
                .into(),
        )
        .structure(
            StructureBuilder::new("ListCitiesOutput")
                .string("nextToken")
                .add_member(
                    MemberBuilder::new("items", "CitySummaries")
                        .required()
                        .into(),
                )
                .into(),
        )
        .list(ListBuilder::new("CitySummaries", "CitySummary"))
        .structure(
            StructureBuilder::new("CitySummary")
                .apply_trait(traits::references(
                    ArrayBuilder::default()
                        .push(
                            ObjectBuilder::default()
                                .reference("resource", "City")
                                .into(),
                        )
                        .into(),
                ))
                .add_member(MemberBuilder::new("cityId", "CityId").required().into())
                .add_member(MemberBuilder::string("name").required().into())
                .into(),
        )
        .operation(
            OperationBuilder::new("GetCurrentTime")
                .readonly()
                .output("GetCurrentTimeOutput")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetCurrentTimeOutput")
                .add_member(MemberBuilder::timestamp("time").required().into())
                .into(),
        )
        .operation(
            OperationBuilder::new("GetForecast")
                .readonly()
                .input("GetForecastInput")
                .output("GetForecastOutput")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetForecastInput")
                .add_member(MemberBuilder::new("cityId", "CityId").required().into())
                .into(),
        )
        .structure(
            StructureBuilder::new("GetForecastOutput")
                .float("chanceOfRain")
                .into(),
        )
        .into()
}

#[test]
fn write_weather_example() {
    let mut writer = SmithyWriter::new(NamespaceID::new_unchecked("example.weather"));
    let model = make_weather_model();
    let output = write_model_to_string(&mut writer, &model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}
