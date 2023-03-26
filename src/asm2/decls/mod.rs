use crate::*;


mod bank;
mod bankdef;
mod ruledef;
mod symbol;


#[derive(Debug)]
pub struct ItemDecls
{
    pub bankdefs: util::SymbolManager<asm2::Bankdef>,
    pub ruledefs: util::SymbolManager<asm2::Ruledef>,
    pub symbols: util::SymbolManager<asm2::Symbol>,
}


pub fn collect(
    report: &mut diagn::Report,
    ast: &mut asm2::AstTopLevel)
    -> Result<ItemDecls, ()>
{
    let mut collections = ItemDecls {
        bankdefs: util::SymbolManager::new("bank"),
        ruledefs: util::SymbolManager::new("ruledef"),
        symbols: util::SymbolManager::new("symbol"),
    };


    bankdef::collect(report, ast, &mut collections)?;
    bank::collect(report, ast, &mut collections)?;
    ruledef::collect(report, ast, &mut collections)?;
    symbol::collect(report, ast, &mut collections)?;

    report.stop_at_errors()?;


    Ok(collections)
}