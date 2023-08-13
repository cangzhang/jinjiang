// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod novels
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertNovelParams < T1 : cornucopia_async::StringSql,> { pub novel_id : i32,pub author_id : i32,pub title : T1,}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct Novel
{ pub id : i32,pub novel_id : i32,pub author_id : i32,pub title : String,}pub struct NovelBorrowed < 'a >
{ pub id : i32,pub novel_id : i32,pub author_id : i32,pub title : &'a str,} impl < 'a > From < NovelBorrowed <
'a >> for Novel
{
    fn
    from(NovelBorrowed { id,novel_id,author_id,title,} : NovelBorrowed < 'a >)
    -> Self { Self { id,novel_id,author_id,title: title.into(),} }
}pub struct NovelQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> NovelBorrowed,
    mapper : fn(NovelBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > NovelQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(NovelBorrowed) -> R) -> NovelQuery
    < 'a, C, R, N >
    {
        NovelQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct InsertNovel
{ pub id : i32,pub novel_id : i32,pub author_id : i32,pub title : String,pub created_at : String,pub updated_at : String,}pub struct InsertNovelBorrowed < 'a >
{ pub id : i32,pub novel_id : i32,pub author_id : i32,pub title : &'a str,pub created_at : &'a str,pub updated_at : &'a str,} impl < 'a > From < InsertNovelBorrowed <
'a >> for InsertNovel
{
    fn
    from(InsertNovelBorrowed { id,novel_id,author_id,title,created_at,updated_at,} : InsertNovelBorrowed < 'a >)
    -> Self { Self { id,novel_id,author_id,title: title.into(),created_at: created_at.into(),updated_at: updated_at.into(),} }
}pub struct InsertNovelQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> InsertNovelBorrowed,
    mapper : fn(InsertNovelBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > InsertNovelQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(InsertNovelBorrowed) -> R) -> InsertNovelQuery
    < 'a, C, R, N >
    {
        InsertNovelQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn get_novels() -> GetNovelsStmt
{ GetNovelsStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
    id, 
    novel_id,
    author_id,
    title
FROM novels")) } pub
struct GetNovelsStmt(cornucopia_async :: private :: Stmt) ; impl
GetNovelsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> NovelQuery < 'a, C,
Novel, 0 >
{
    NovelQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { NovelBorrowed { id : row.get(0),novel_id : row.get(1),author_id : row.get(2),title : row.get(3),} }, mapper : | it | { <Novel>::from(it) },
    }
} }pub fn insert_novel() -> InsertNovelStmt
{ InsertNovelStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO novels (
    novel_id,
    author_id,
    title
) VALUES (
    $1,
    $2,
    $3
) ON CONFLICT (novel_id) 
DO
    UPDATE SET author_id = $2, title = $3 
RETURNING *")) } pub
struct InsertNovelStmt(cornucopia_async :: private :: Stmt) ; impl
InsertNovelStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
novel_id : & 'a i32,author_id : & 'a i32,title : & 'a T1,) -> InsertNovelQuery < 'a, C,
InsertNovel, 3 >
{
    InsertNovelQuery
    {
        client, params : [novel_id,author_id,title,], stmt : & mut self.0, extractor :
        | row | { InsertNovelBorrowed { id : row.get(0),novel_id : row.get(1),author_id : row.get(2),title : row.get(3),created_at : row.get(4),updated_at : row.get(5),} }, mapper : | it | { <InsertNovel>::from(it) },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, InsertNovelParams < T1,>, InsertNovelQuery < 'a,
C, InsertNovel, 3 >, C > for InsertNovelStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertNovelParams < T1,>) -> InsertNovelQuery < 'a, C,
    InsertNovel, 3 >
    { self.bind(client, & params.novel_id,& params.author_id,& params.title,) }
}}}