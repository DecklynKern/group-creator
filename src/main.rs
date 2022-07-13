use eframe;
use rand;
use rand::seq::SliceRandom;

fn main() {

    let options = eframe::NativeOptions::default();
    
    let window = GroupCreator {
        names: Vec::new(),
        num_groups: String::from("3"),
        generated_groups: Vec::new(),
    };

    eframe::run_native(
        "Group Creator",
        options,
        Box::new(|_| Box::new(window)),
    );

}

struct GroupCreator {
    names: Vec<String>,
    num_groups: String,
    generated_groups: Vec<Vec<String>>,
}

impl eframe::App for GroupCreator {

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        
        eframe::egui::SidePanel::left("noteslist").show(ctx, |ui| {

            ui.horizontal(|ui| {

                ui.label("Number of groups: ");
                ui.text_edit_singleline(&mut self.num_groups);

            });

            ui.horizontal(|ui| {

                if ui.button("Add").clicked() {
                    self.names.push(String::from("new person"));
                }

                if ui.button("Generate Groups").clicked() {
                    self.generate_groups();
                }
            });

            ui.separator();

            eframe::egui::ScrollArea::vertical().show(ui, |ui| {

                let mut del: Option<usize> = None;

                for (idx, name) in self.names.iter_mut().enumerate() {

                    ui.horizontal(|ui| {

                        let name_entry = eframe::egui::widgets::TextEdit::singleline(name)
                            .desired_width(150.0);

                        ui.add(name_entry);

                        if ui.button("X").clicked() {
                            del = Some(idx);
                        }

                    });
                }

                if del.is_some() {
                    self.names.remove(del.unwrap());
                }

            });
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| {

                for (idx, group) in self.generated_groups.iter().enumerate() {
                    
                    ui.separator();

                    ui.vertical(|ui| {

                        ui.label(format!("Group {}", idx + 1));

                        for person in group {

                            ui.label(person);
        
                        }
                    });
                }
            });
        });
    }
}

impl GroupCreator {

    fn generate_groups(&mut self) {
        
        self.generated_groups.clear();

        let groups_num = self.num_groups.parse::<usize>().unwrap();

        for _group in 0..groups_num {

            self.generated_groups.push(Vec::new());

        }

        let mut rng = rand::thread_rng();

        let mut names_copy = self.names.clone();                        
        names_copy.as_mut_slice().shuffle(&mut rng);

        let mut group_index = 0;

        for name in names_copy {

            self.generated_groups[group_index].push(name);

            group_index += 1;
            group_index %= groups_num;

        }
    }
}