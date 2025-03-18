#[macro_export]
macro_rules! command_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
           $($variant_name:ident($variant_ty:ty)$(,)?)*
        }
    ) => {
        command_enum!(
            $(#[$meta])*
            $vis enum $name {
               $($variant_name($variant_ty)),*
            }
            pub async fn run(self) -> Outcome {}
        );
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
           $($variant_name:ident($variant_ty:ty)$(,)?)*
        }
        $fun_vis:vis async fn $fun_id:ident(self) $(-> $ret:ty)? {}
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant_name($variant_ty)),*
        }

        impl $name {
            $fun_vis async fn $fun_id(self) $(-> $ret)? {
                match self {
                    $(Self::$variant_name(command) => command.$fun_id().await),*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::io;

    type Outcome<T = ()> = io::Result<T>;

    #[derive(Default, Clone, Debug)]
    struct PrintCommand {
        input: String,
    }

    impl PrintCommand {
        pub async fn run(self) -> Outcome {
            println!("{}", self.input);
            Ok(())
        }
    }

    #[derive(Default, Clone, Debug)]
    struct ActCommand {
        timeout: u32,
    }

    impl ActCommand {
        pub async fn run(self) -> Outcome {
            println!("{}", self.timeout);
            Ok(())
        }
    }

    command_enum!(
        #[derive(Clone, Debug)]
        enum Command {
            Print(PrintCommand),
            Act(ActCommand),
        }
    );

    impl Command {
        pub fn print() -> Self {
            Self::Print(PrintCommand::default())
        }

        pub fn act() -> Self {
            Self::Act(ActCommand::default())
        }
    }

    #[tokio::test]
    async fn must_run_command() -> Outcome {
        Command::print().run().await?;
        Command::act().run().await?;
        Ok(())
    }
}
