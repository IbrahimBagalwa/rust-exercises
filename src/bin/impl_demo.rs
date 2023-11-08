struct Temperature{
    fahrenheit: f64,
}

impl Temperature{
    fn freezing()-> Self {
        Self{
            fahrenheit: 32.0,
        }
    }

    fn boiling()-> Self {
        Self{
            fahrenheit: 212.0,
        }
    }

    fn get_temperature(&self){
        println!("{:?} degrees F", self.fahrenheit);
    }
}

fn main(){
    let temperature  =  Temperature{
        fahrenheit: 9.23
    };
    temperature.get_temperature();

    let cold = Temperature::freezing();
    cold.get_temperature();

    let boiling = Temperature::boiling();
    boiling.get_temperature();
}