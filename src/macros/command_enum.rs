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
            pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome {}
        );
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
           $($variant_name:ident($variant_ty:ty)$(,)?)*
        }
        $fun_vis:vis async fn $fun_id:ident(self, stdout: &mut impl Write, stderr: &mut impl Write) $(-> $ret:ty)? {}
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant_name($variant_ty)),*
        }

        impl $name {
            $fun_vis async fn $fun_id(self, stdout: &mut impl Write, stderr: &mut impl Write) $(-> $ret)? {
                match self {
                    $(Self::$variant_name(command) => command.$fun_id(stdout, stderr).await),*
                }
            }
        }
    };
}
