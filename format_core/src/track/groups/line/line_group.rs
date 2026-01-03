use vector2d::Vector2Df;

use crate::track::line::{
    acceleration_line::{AccelerationLine, AccelerationLineBuilder},
    scenery_line::{SceneryLine, SceneryLineBuilder},
    standard_line::{StandardLine, StandardLineBuilder},
};

#[derive(PartialEq, Debug)]
pub struct LineGroup {
    standard_lines: Vec<StandardLine>,
    acceleration_lines: Vec<AccelerationLine>,
    scenery_lines: Vec<SceneryLine>,
}

impl LineGroup {
    pub fn standard_lines(&self) -> &Vec<StandardLine> {
        &self.standard_lines
    }

    pub fn acceleration_lines(&self) -> &Vec<AccelerationLine> {
        &self.acceleration_lines
    }

    pub fn scenery_lines(&self) -> &Vec<SceneryLine> {
        &self.scenery_lines
    }
}

pub struct LineGroupBuilder {
    standard_lines: Vec<StandardLineBuilder>,
    acceleration_lines: Vec<AccelerationLineBuilder>,
    scenery_lines: Vec<SceneryLineBuilder>,
}

impl LineGroupBuilder {
    pub fn new() -> Self {
        Self {
            standard_lines: Vec::new(),
            acceleration_lines: Vec::new(),
            scenery_lines: Vec::new(),
        }
    }

    pub fn add_standard_line(
        &mut self,
        id: u32,
        endpoints: (Vector2Df, Vector2Df),
    ) -> &mut StandardLineBuilder {
        self.standard_lines
            .push(StandardLineBuilder::new(id, endpoints));
        self.standard_lines.last_mut().unwrap()
    }

    pub fn get_standard_lines(&mut self) -> &mut Vec<StandardLineBuilder> {
        &mut self.standard_lines
    }

    pub fn add_acceleration_line(
        &mut self,
        id: u32,
        endpoints: (Vector2Df, Vector2Df),
    ) -> &mut AccelerationLineBuilder {
        self.acceleration_lines
            .push(AccelerationLineBuilder::new(id, endpoints));

        self.acceleration_lines.last_mut().unwrap()
    }

    pub fn get_acceleration_lines(&mut self) -> &mut Vec<AccelerationLineBuilder> {
        &mut self.acceleration_lines
    }

    pub fn add_scenery_line(
        &mut self,
        endpoints: (Vector2Df, Vector2Df),
    ) -> &mut SceneryLineBuilder {
        self.scenery_lines.push(SceneryLineBuilder::new(endpoints));
        self.scenery_lines.last_mut().unwrap()
    }

    pub fn scenery_lines(&mut self) -> &mut Vec<SceneryLineBuilder> {
        &mut self.scenery_lines
    }

    pub(crate) fn build(&self) -> LineGroup {
        let mut standard_lines: Vec<StandardLine> = vec![];
        let mut acceleration_lines: Vec<AccelerationLine> = vec![];
        let mut scenery_lines: Vec<SceneryLine> = vec![];

        for standard_line_builder in &self.standard_lines {
            let standard_line = standard_line_builder.build();
            standard_lines.push(standard_line);
        }

        for acceleration_line_builder in &self.acceleration_lines {
            let acceleration_line = acceleration_line_builder.build();
            acceleration_lines.push(acceleration_line);
        }

        for scenery_line_builder in &self.scenery_lines {
            let scenery_line = scenery_line_builder.build();
            scenery_lines.push(scenery_line);
        }

        standard_lines.sort_by_key(|line| line.id());
        acceleration_lines.sort_by_key(|line| line.id());
        LineGroup {
            standard_lines,
            acceleration_lines,
            scenery_lines,
        }
    }
}
