use std::path::Path;

use crate::v70;


pub fn file_path_to_planet_index(file_path: &Path) -> Option<v70::PlanetType> {
	let file_name = file_path.file_stem()?.to_str()?;
	let planet_name = file_name.split('.').next()?
		.to_ascii_uppercase();

	Some(match planet_name.as_str() {
		"SNOW_1" => v70::PlanetType::Snowcrab,
		"SNOW_2" => v70::PlanetType::Frigia,
		"SNOW_3" => v70::PlanetType::IceBerg,
		"SNOW_4" => v70::PlanetType::TheCooler,
		"SNOW_5" => v70::PlanetType::UltimaThule,
		"SNOW_6" => v70::PlanetType::LongFloes,
		"CRATER_1" => v70::PlanetType::IronCross,
		"CRATER_2" => v70::PlanetType::Splatterscape,
		"CRATER_3" => v70::PlanetType::Peakaboo,
		"CRATER_4" => v70::PlanetType::ValentinesPlanet,
		"CRATER_5" => v70::PlanetType::ThreeRings,
		"CRATER_6" => v70::PlanetType::GreatDivide,
		"GREEN_1" => v70::PlanetType::NewLuzon,
		"GREEN_2" => v70::PlanetType::MiddleSea,
		"GREEN_3" => v70::PlanetType::HighImpact,
		"GREEN_4" => v70::PlanetType::Sanctuary,
		"GREEN_5" => v70::PlanetType::Islandia,
		"GREEN_6" => v70::PlanetType::Hammerhead,
		"DESERT_1" => v70::PlanetType::Freckles,
		"DESERT_2" => v70::PlanetType::Sandspit,
		"DESERT_3" => v70::PlanetType::GreatCircle,
		"DESERT_4" => v70::PlanetType::LongPassage,
		"DESERT_5" => v70::PlanetType::FlashPoint,
		"DESERT_6" => v70::PlanetType::Bottleneck,
		_ => return None,
	})
}
