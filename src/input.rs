/// Хранит необходимое кол-во нулей в хеше (n) и
/// требуемое кол-во хешей (f)
#[derive(Debug, PartialEq)]
pub struct Config {
    pub n: u32,
    pub f: u32,
}

struct InputParams(Option<u32>, Option<u32>);
enum InputParam {
    N(u32),
    F(u32),
}

impl InputParam {
    fn from(param: String, value: u32) -> Result<Self, String> {
        if param == "-N" {
            return Ok(Self::N(value));
        }
        if param == "-F" {
            return Ok(Self::F(value));
        }

        return Err(format!("Invalid param {param}"));
    }
}

const REQUIRED_PARAMS: [&str; 2] = ["-N", "-F"];

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, String> {
        let params = Self::parse_params(args)?;

        Self::validate_params(params)
    }

    /// Преобразование InputParams в Config, в данной ситуации ошибки из этой функции
    /// не будут возвращены никогда, однако сохраняю данную структуру ввиду возможности
    /// дальнейшего изменнения логики обработки входящих параметров
    /// (например, при отсутствии, можно устанавливать значение по умолчанию)
    fn validate_params(params: InputParams) -> Result<Config, String> {
        if let Some(n) = params.0 {
            if let Some(f) = params.1 {
                Ok(Config { n, f })
            } else {
                Err(String::from("You should provide F param"))
            }
        } else {
            Err(String::from("You should provide N param"))
        }
    }

    /// Получение и проверка переданных параметров на уникальность
    fn parse_params(mut args: impl Iterator<Item = String>) -> Result<InputParams, String> {
        let mut n: Option<u32> = None;
        let mut f: Option<u32> = None;

        let p1 = Self::read_param(&mut args)?;
        let p2 = Self::read_param(&mut args)?;

        for i in [p1, p2] {
            match i {
                InputParam::N(v) => {
                    if n.is_some() {
                        return Err(String::from("You should provide each parameter just once!"));
                    }
                    n = Some(v);
                }
                InputParam::F(v) => {
                    if f.is_some() {
                        return Err(String::from("You should provide each parameter just once!"));
                    }
                    f = Some(v);
                }
            }
        }

        Ok(InputParams(n, f))
    }

    /// Чтение параметра и его значения из итератора и проверка на их существование
    fn read_param(mut args: impl Iterator<Item = String>) -> Result<InputParam, String> {
        if let Some(arg) = args.next() {
            if REQUIRED_PARAMS.contains(&arg.as_str()) {
                let param = arg;

                if let Some(arg) = args.next() {
                    if let Ok(num) = arg.parse::<u32>() {
                        return InputParam::from(param, num);
                    } else {
                        return Err(format!("You should provide a valid {param} param"));
                    }
                } else {
                    return Err(format!("You should provide a valid {param} param"));
                }
            }
        }
        return Err(String::from("You should provide required parameters N and F"));
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn return_valid_config() {
        let params = [
            String::from("-N"),
            String::from("3"),
            String::from("-F"),
            String::from("4"),
        ].into_iter();

        let res = Config::build(params);
        assert!(res.is_ok());
        assert!(res.is_ok_and(|v| v.n == 3 && v.f == 4));
    }

    #[test]
    fn return_err_on_invalid_params() {
        let params = [].into_iter();
        let res = Config::build(params);
        assert!(res.is_err_and(|v| v.eq("You should provide required parameters N and F")));

        let params = [String::from("-N"), String::from("3"), String::from("-F")].into_iter();
        let res = Config::build(params);
        assert!(res.is_err_and(|v| v.eq("You should provide a valid -F param")));

        let params = [String::from("-N"), String::from("-F"), String::from("4")].into_iter();
        let res = Config::build(params);
        assert!(res.is_err_and(|v| v.eq("You should provide a valid -N param")));

        let params = [
            String::from("-N"),
            String::from("3"),
            String::from("-N"),
            String::from("4"),
        ].into_iter();
        let res = Config::build(params);
        assert!(res.is_err_and(|v| v.eq("You should provide each parameter just once!")));
    }
}
