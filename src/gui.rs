slint::slint! {
    import { AboutSlint, CheckBox, ComboBox, SpinBox, TabWidget,
        VerticalBox, HorizontalBox } from "std-widgets.slint";

    smtk_app := Window {
        title: "Show Me The Key";

        VerticalBox {
            spacing: 10px;
            padding: 10px;

            HorizontalBox {
                spacing: 10px;
                alignment: end;
    
                enable-toggle := CheckBox {
                    checked: false;
                    text: "Enable";
                }
            }

            TabWidget {
                Tab {
                    title: "Controls";

                    VerticalBox {
                        HorizontalBox {
                            spacing: 10px;
                            alignment: space-between;
            
                            Text {
                                vertical-alignment: center;
                                text: "Temporary Hide";
                            }
            
                            CheckBox { }
                        }
            
                        HorizontalBox {
                            spacing: 10px;
                            alignment: space-between;
            
                            Text {
                                vertical-alignment: center;
                                text: "Show Mouse Button";
                            }
            
                            CheckBox {
                                checked: true;
                            }
                        }
            
                        HorizontalBox {
                            spacing: 10px;
                            alignment: space-between;
            
                            Text {
                                vertical-alignment: center;
                                text: "Display Mode";
                            }
            
                            ComboBox {
                                model: ["Composed", "Raw"];
                                current-value: "Raw";
                            }
                        }
            
                        HorizontalBox {
                            spacing: 10px;
                            alignment: space-between;
            
                            Text {
                                vertical-alignment: center;
                                text: "Width (px)";
                            }
            
                            SpinBox {
                                maximum: 3000;
                                value: 1500;
                            }
                        }
            
                        HorizontalBox {
                            spacing: 10px;
                            alignment: space-between;
            
                            Text {
                                vertical-alignment: center;
                                text: "Height (px)";
                            }
            
                            SpinBox {
                                maximum: 1000;
                                value: 200;
                            }
                        }
            
                        HorizontalBox {
                            spacing: 10px;
                            alignment: space-between;
            
                            Text {
                                vertical-alignment: center;
                                text: "Timeout (ms)";
                            }
            
                            SpinBox {
                                value: 0;
                            }
                        }
                    }
                }

                Tab {
                    title: "About";

                    VerticalBox {
                        spacing: 10px;

                        AboutSlint {}
                    }
                }
            }
            
        }
    }
}
