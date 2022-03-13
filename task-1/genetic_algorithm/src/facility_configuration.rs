pub struct FacilityConfig {
    layout: Layout,
    pub dimensions: Dimensions,
}

impl FacilityConfig {
    fn new(
        flow_path: String,
        cost_path: String,
        width: u64,
        height: u64,
        machines: Vec<u64>,
    ) -> Result<Self, &'static str> {
        if machines.len() > (width * height) as usize {
            return Err("Width * height must be greater than the machine count.");
        }

        Ok(FacilityConfig {
            layout: Layout {
                flow_path,
                cost_path,
            },
            dimensions: Dimensions {
                width,
                height,
                machines,
            },
        })
    }

    // TODO: could this somehow be made const?
    pub fn get_easy_config() -> Result<Self, &'static str> {
        FacilityConfig::new(
            String::from("data/easy_flow.json"),
            String::from("data/easy_cost.json"),
            3,
            3,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        )
    }

    pub fn get_flow_path(&self) -> &String {
        &self.layout.flow_path
    }

    pub fn get_cost_path(&self) -> &String {
        &self.layout.cost_path
    }
}

struct Layout {
    pub flow_path: String,
    pub cost_path: String,
}

pub struct Dimensions {
    pub width: u64,
    pub height: u64,
    pub machines: Vec<u64>,
}
