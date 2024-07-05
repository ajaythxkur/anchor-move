use anchor_lang::prelude::*;

declare_id!("BjQpkatN2jkWfEESMLvScqTDgvrxGtQw5Krj6ftmefQQ");

const CONTENT_MAX_LEN: usize = 1024;
const APP_SEED: &str = "todo";

#[program]
pub mod todo_list {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let todolist = &mut ctx.accounts.todolist;
        todolist.bump = ctx.bumps.todolist;
        Ok(())
    }
    pub fn create_task(ctx: Context<CreateTask>, content: String) -> Result<()> {
        let todolist = &mut ctx.accounts.todolist;
        let task = &mut ctx.accounts.task;
        todolist.last_task_id = todolist.last_task_id.checked_add(1).unwrap();
        task.task_id = todolist.last_task_id;
        task.content = content;
        task.signer = ctx.accounts.signer.key();
        task.completed = false;
        task.bump = ctx.bumps.task;
        Ok(())
    }
    pub fn complete_task(ctx: Context<CompleteTask>, _task_id: u64) -> Result<()>{
        let task = &mut ctx.accounts.task;
        task.completed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init,
        payer = signer,
        seeds = [APP_SEED.as_bytes(), signer.key().as_ref()],
        bump,
        space = 8 + 8
    )]
    pub todolist: Account<'info, TodoList>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct TodoList{
    pub last_task_id: u64,
    pub bump: u8
}

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(
        mut,
        seeds = [APP_SEED.as_bytes(), signer.key().as_ref()],
        bump = todolist.bump,
    )]
    pub todolist: Account<'info, TodoList>,
    #[account(
        init, 
        payer = signer, 
        seeds=[todolist.key().as_ref(), &(todolist.last_task_id + 1).to_le_bytes()], 
        bump, 
        space = 8 + 32 + CONTENT_MAX_LEN + 1 + 1
    )]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct CompleteTask<'info> {
    #[account(
        mut,
        seeds = [APP_SEED.as_bytes(), signer.key().as_ref()],
        bump = todolist.bump,
    )]
    pub todolist: Account<'info, TodoList>,
    #[account(
        mut, 
        seeds=[todolist.key().as_ref(), &(task_id).to_le_bytes()], 
        bump = task.bump, 
        constraint = task.completed == false
    )]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
pub struct Task{
    pub task_id: u64,
    pub content: String,
    pub completed: bool,
    pub signer: Pubkey,
    pub bump: u8,
}

// TODO: Write tests in /tests/todo-list.ts