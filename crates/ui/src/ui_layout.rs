slint::slint! {
    import { ScrollView, TextEdit } from "std-widgets.slint";

    export struct TabItem {
        name: string,
        active: bool,
    }

    component TabBar inherits HorizontalLayout {
        in property <[TabItem]> tabs;
        in property <int> active_index;
        
        spacing: 2px;
        height: 24px;

        for tab[idx] in tabs: Rectangle {
            width: 100px;
            background: tab.active ? #2d2d2d : transparent;
            
            TouchArea {
                clicked => { root.tab_selected(idx); }
            }
            
            Rectangle {
                x: 8px;
                background: transparent;
                Text {
                    text: tab.name;
                    color: tab.active ? #ffffff : #cccccc;
                    font-size: 12px;
                    vertical-alignment: center;
                }
            }
        }
        
        Rectangle { } // Spacer to fill remaining space
        
        callback tab_selected(int);
    }

    component FileList inherits ScrollView {
        in property <[string]> files;
        
        callback open_file(string);

        VerticalLayout {
            padding: 4px;
            for file in root.files : TouchArea {
                clicked => { root.open_file(file); }
                Rectangle {
                    height: 20px;
                    background: #2a2a2a;
                    Text {
                        text: file;
                        color: #cccccc;
                        font-size: 11px;
                        vertical-alignment: center;
                    }
                }
            }
        }
    }

    component Editor inherits Rectangle {
        in-out property <string> text;
        
        background: #1e1e1e;
        
        callback text_changed(string);

        ScrollView {
            TextEdit {
                text <=> root.text;
                edited => { root.text_changed(self.text); }
                font-size: 13px;
                wrap: word-wrap;
            }
        }
    }

    component StatusBar inherits Rectangle {
        in property <string> title;
        
        height: 20px;
        background: #252525;
        
        HorizontalLayout {
            spacing: 8px;
            padding: 4px;
            Text {
                text: "Ln 1, Col 1";
                color: #888888;
                font-size: 11px;
            }
            Rectangle { } // Spacer
            Text {
                text: root.title;
                color: #888888;
                font-size: 11px;
            }
        }
    }

    export component MainWindow inherits Window {
        width: 960px;
        height: 640px;

        in-out property <string> window_title: "EmberIDE";
        title: root.window_title;

        in-out property <[TabItem]> tabs;
        in property <int> active_tab_index;
        
        in-out property <string> editor_text;
        in property <[string]> files;

        in property <bool> command_palette_open;
        in property <[string]> command_list;

        callback open_file(string);
        callback save_file();
        callback editor_changed(string);
        callback tab_selected(int);
        callback tab_closed(int);
        callback toggle_command_palette();
        callback close_command_palette();
        callback command_executed(string);

        FocusScope {
            key-pressed(event) => {
                if (event.modifiers.control) {
                    if (event.text == "s") { 
                        root.save_file(); 
                        accept 
                    }
                    else if (event.text == "p") { 
                        root.toggle_command_palette(); 
                        accept 
                    }
                    else if (event.text == "w") { 
                        if (root.active_tab_index >= 0) { 
                            root.tab_closed(root.active_tab_index); 
                        }
                        accept 
                    }
                    else { 
                        reject 
                    }
                } else { 
                    reject 
                }
            }

            VerticalLayout {
                padding: 0px;
                spacing: 0px;

                // Tab bar
                TabBar {
                    tabs: root.tabs;
                    active_index: root.active_tab_index;
                    tab_selected(idx) => { root.tab_selected(idx); }
                }

                // Main content area
                HorizontalLayout {
                    // Sidebar
                    Rectangle {
                        width: 160px;
                        background: #252525;
                        FileList {
                            files: root.files;
                            open_file(file) => { root.open_file(file); }
                        }
                    }

                    // Editor
                    Editor {
                        text <=> root.editor_text;
                        text_changed(text) => { root.editor_changed(text); }
                    }
                }
            }
        }

        // Status bar at bottom
        StatusBar {
            title: root.window_title;
        }
    }
}


