impl Object {
    pub fn new(center_point: Point, obj_type: ObjType) -> Self {
        println!(
            "[DEBUG] Creating new object with center: {:?}, type: {:?}",
            center_point, obj_type
        );
        Self {
            center_point,
            obj_type,
            allocated_box: None,
        }
    }

    pub fn allocate_from_display(&mut self, display: &Display) {
        println!("[DEBUG] Allocating object {:?} from display", self);
        display.initialize_object(self);
        if self.is_initialized() {
            println!("[DEBUG] Allocation successful for object: {:?}", self);
        } else {
            println!("[ERROR] Allocation failed for object: {:?}", self);
        }
    }

    pub fn fill_box(&mut self, new_ch: char) {
        println!(
            "[DEBUG] Filling allocated box for object {:?} with '{}'",
            self, new_ch
        );
        match self.allocated_box {
            Some(ref allocated_box) => {
                println!("[DEBUG] Allocated box size: {:?}", allocated_box.size());
                allocated_box.vec.iter().for_each(|row| {
                    row.iter().for_each(|data_point| {
                        data_point.get_ref().update(new_ch);
                    });
                });
            }
            None => println!("[ERROR] Cannot fill box: allocated box is None"),
        }
    }

    pub fn get_allocated_box(&self) -> &Vec2<Ptr<DataPoint>> {
        if let Some(ref allocated_box) = self.allocated_box {
            println!(
                "[DEBUG] Accessing allocated box for object {:?}",
                self
            );
            allocated_box
        } else {
            panic!("[ERROR] {}", ERROR_OBJECT_EMPTY_BOX);
        }
    }

    pub fn set_allocated_box(&mut self, new_box: Vec2<Ptr<DataPoint>>) {
        println!(
            "[DEBUG] Setting allocated box for object {:?} with size {:?}",
            self,
            new_box.size()
        );
        self.allocated_box = Some(new_box);
    }
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        println!("[DEBUG] Creating new display: width = {}, height = {}", width, height);
        Display {
            screen: Vec2::new(width, height),
            width,
            height,
            boxer: Vec::new(),
        }
    }

    pub fn pixel(&mut self, point: Point, new_val: char) {
        println!("[DEBUG] Placing pixel at {:?} with '{}'", point, new_val);
        if point.x < self.width && point.y < self.height {
            self.screen.index(point.x, point.y).update(new_val);
        } else {
            println!(
                "[ERROR] Point {:?} is out of display bounds (width: {}, height: {})",
                point, self.width, self.height
            );
        }
    }

    pub fn add_object(&mut self, mut object: Object) {
        println!("[DEBUG] Adding object to display: {:?}", object);
        object.allocate_from_display(self);
        if object.is_initialized() {
            println!("[DEBUG] Object added successfully: {:?}", object);
            self.boxer.push(object);
        } else {
            println!("[ERROR] Failed to add object: {:?}", object);
        }
    }

    pub fn initialize_object(&self, obj_ref: &mut Object) {
        println!("[DEBUG] Initializing object {:?} from display", obj_ref);
        obj_ref.set_allocated_box(self.allocate(2, 10, 10, 2));
        if obj_ref.is_initialized() {
            println!("[DEBUG] Object initialized successfully: {:?}", obj_ref);
        } else {
            println!("[ERROR] Failed to initialize object: {:?}", obj_ref);
        }
    }

    pub fn allocate(
        &self,
        left: usize,
        right: usize,
        top: usize,
        bottom: usize,
    ) -> Vec2<Ptr<DataPoint>> {
        println!(
            "[DEBUG] Allocating box in region left={}, right={}, top={}, bottom={}",
            left, right, top, bottom
        );

        let right = if right >= self.width {
            self.width - 1
        } else {
            right
        };
        let top = if top >= self.height {
            self.height - 1
        } else {
            top
        };

        if right < left {
            panic!("[ERROR] Invalid allocation region: right < left");
        }
        if top < bottom {
            panic!("[ERROR] Invalid allocation region: top < bottom");
        }

        let width = right - left + 1;
        let height = top - bottom + 1;

        println!(
            "[DEBUG] Allocating box with dimensions width={}, height={}",
            width, height
        );

        let mut reference_vec2 = Vec2::new(width, height);
        for y in bottom..=top {
            for x in left..=right {
                reference_vec2.vec[y - bottom][x - left] =
                    Ptr::new_from_ptr(&self.screen.vec[y][x] as *const DataPoint as *mut DataPoint);
            }
        }
        println!("[DEBUG] Allocation completed successfully");
        reference_vec2
    }
}
