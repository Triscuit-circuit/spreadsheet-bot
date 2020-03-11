//use "*" to configure bot
use serenity::{
    prelude::*,
    model::{prelude::*,channel::{Message,Embed}},
    framework::standard::{
        Args,CommandResult,
        CommandOptions,CommandGroup, DispatchError,
        HelpOptions,help_commands,StandardFramework,
        macros::{command,group,help,check}
    },
    utils::{content_safe,ContentSafeOptions,MessageBuilder},
    client::bridge::gateway::{ShardManager,ShardId}
};
use std::{collections::{HashSet},
          env,fmt::write,
          sync::Arc};
use crate::commands::spreadsheet;

pub(crate) struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}

#[command]
fn servers(ctx: &mut Context,msg:&Message)->CommandResult{
    let string = ctx.clone();
    let input = &msg.content;
    let mut input_arr:Vec<String> = input.splitn(2," ").map(|x| x.to_string()).collect();
    let test = &string.cache.read().guilds;
    let mut trt:String = "".to_string();
    if input_arr.len() >= 2{
        println!("{:#?}",input_arr);
        let input_two = &input_arr[1];
        if &input_two[0..1] == "\"" && &input_two[input_two.len()-1..input_two.len()] == "\""{
            let server_named = &input_two[1..input_two.len()-1];
            for (guild,arc) in test{
                if arc.read().name.eq(server_named){
                    let mut response = MessageBuilder::new();
                    for (userid,username) in &arc.read().members{
                        response.push(format!(" userid:`{}` username:`{}`\n",userid,username.user.read().name));
                        println!("{}",format!(" userid:`{}` username:`{}`\n",userid,username.user.read().name));
                        for f in &username.roles{
                            //response.push(format!("roles: {}\n",f.to_role_cached(&ctx.cache).unwrap().name));
                            println!("{}",format!("roles: {}\n",f.to_role_cached(&ctx.cache).unwrap().name));
                        }
                    }
                    if let Err(why) =msg.channel_id.say(&ctx.http,&response){
                        println!("Error sending message: {:?}",why);
                    };
                }
            }
        } else{
            if let Err(why) =  msg.reply(ctx,format!("{}","``` Error parsing server name, please enter with quotes,")){
                println!("Error sending message: {:?}",why);
            };
        }
    }else{
        for val in test{
            trt = format!("{}\n> {}", trt, val.1.read().name);
        }
        println!("{}",trt);

        if let Err(why) =  msg.reply(ctx,format!("{}",trt)){
            println!("Error sending message: {:?}",why);
        };
    }
    Ok(())
}
#[command]
fn ping(ctx: &mut Context, msg: &Message)-> CommandResult{
    let data = ctx.data.read();
    let shard_manager = match data.get::<ShardManagerContainer>(){
        Some(t) => t,
        None =>{
            let _ = msg.reply(&ctx, "there was a problem with the shard manager");
            return Ok(());
        }
    };
    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    let runner = match runners.get(&ShardId(ctx.shard_id)){
        Some(runner)=> runner,
        None=>{
            let _ = msg.reply(&ctx,"No Shard Found");
            return Ok(());
        }
    };
    let _ = msg.reply(&ctx, &format!("The shard latency is {:?}", runner.latency));
    Ok(())

}
#[command]
#[owners_only]
fn inter_roles(ctx: &mut Context, msg: &Message)-> CommandResult{
    //TODO
    Ok(())
}
#[command]
#[owners_only]
fn config(ctx: &mut Context, msg: &Message)-> CommandResult{
    //TODO
    Ok(())
}
#[command]
fn about(ctx: &mut Context, msg: &Message)-> CommandResult{
    let response = MessageBuilder::new()
        .push_quote_line("Spreadsheet bot creator: Chilla#4568")
        .push_quote_line("Discord bot API credit: Serenity Team");
    //msg.channel_id.broadcast_typing(ctx);
    if let Err(why) =msg.channel_id.say(&ctx.http,&response){
        println!("Error sending message: {:?}",why);
    };
    Ok(())
}
#[command]
fn spread(ctx: &mut Context, msg: &Message)-> CommandResult{
    let input = &msg.content.replace(";s","");
    let mut l = spreadsheet::enter_command(input.parse().unwrap());
    println!("username:{},command:{}",msg.author.name,msg.content);
    println!("user id:{}, username:{}, spreadsheet \n{}",msg.author.id,msg.author.name,l);
    l = format!("\n```{}```",l);
    if let Err(why) =  msg.reply(ctx,l ){
        println!("Error sending message: {:?}",why);
    };
    Ok(())
}