extern crate colored;
use colored::*;

// message de bienvenue
pub fn welcome(){
    println!("{}", "     -----------------------------------------------------------------------------");
    println!("{}", "     ");
    println!("{}", "     -----------------------------------------------------------------------------");
    println!("{}", "     ");

    println!("{}","
                        _____ _           _     ____            
                        / ____| |         | |   |  _ \\           
                        | |    | |__   __ _| |_  | |_) | _____  __
                        | |    | '_ \\ / _` | __| |  _ < / _ \\ \\/ /
                        | |____| | | | (_| | |_  | |_) | (_) >  < 
                        \\_____|_| |_|\\__,_|\\__| |____/ \\___/_/\\_\\
    ".green());


    println!("{}","
    
                                _                        
                                \\`*-.                    
                                )  _`-.                 
                                .  : `. .                
                                : _   '  \\               
                                ; *` _.   `*-._          
                                `-.-'          `-.       
                                    ;       `       `.     
                                    :.       .        \\    
                                    . \\  .   :   .-'   .   
                                    '  `+.;  ;  '      :   
                                    :  '  |    ;       ;-. 
                                    ; '   : :`-:     _.`* ;
                    [Groupe1] .*' /  .*' ; .*`- +'  `*' 
                                `*-*   `*-*  `*-*'        

    ".bold());

    println!("{}"," 
                    de CUMONT GUILLAUME 4SI1
                    SAKHO Moussa 4SI1
                    DUBO Sullyvan 4SI1
    ".green());

    println!("{}", "                                                © ESGI Groupe 1 - 2021-2022".bold().green());
    println!("{}", "     ");
    println!("{}", "     ------------------------------------------------------------------------");
    println!("{}", "     ");
    println!("{}", "     ------------------------------------------------------------------------");
    }



pub fn client(){
    println!("{}", "
                    ░█████╗░██╗░░░░░██╗███████╗███╗░░██╗████████╗
                    ██╔══██╗██║░░░░░██║██╔════╝████╗░██║╚══██╔══╝
                    ██║░░╚═╝██║░░░░░██║█████╗░░██╔██╗██║░░░██║░░░
                    ██║░░██╗██║░░░░░██║██╔══╝░░██║╚████║░░░██║░░░
                    ╚█████╔╝███████╗██║███████╗██║░╚███║░░░██║░░░
                    ░╚════╝░╚══════╝╚═╝╚══════╝╚═╝░░╚══╝░░░╚═╝░░░
".blink().green().bold());
}