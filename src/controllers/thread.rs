use conf::*;
use iron::prelude::*;
use iron;
use persistent;
use bodyparser;
use persistent::Read;
use iron::status;
use router::Router;
use std::vec::Vec;
use std::string::String;
use std::str::FromStr;
use serde_json::Error;
use postgres::types::ToSql;
use postgres::rows::Row;
#[macro_use]
use serde_derive;
use serde_json;
use params::{Params, Value};
use std::io::copy;
use ijr;
use db;
use ijr::{JsonResponseMiddleware, JsonResponse};
use serde_json::from_str;
use queries::user as u_q;
use models::user as u_model;
use models::error::{ErrorMsg};
use self::u_model::{User, JsonUser, empty_user, copy_user, read_user};
use managers::user_manager as u_manager;
use models::post::*;
use models::thread::*;
use managers::thread_manager::*;
use managers::post_manager as p_m;
use managers::post_manager::*;
use managers::user_manager::*;
use queries::post::*;


pub fn create_posts(request : &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db_pool = &request.get::<persistent::Read<DbPool>>().unwrap();
    let conn = db_pool.get().unwrap();
    let raw = request.get::<bodyparser::Raw>().unwrap().unwrap();
    let slug_or_id = request.extensions.get::<Router>().unwrap().find("slug_or_id").unwrap_or("/");

    let mut thread_option;
    match from_str::<i32>(slug_or_id) {
        Ok(val) => thread_option = get_thread(&val, &conn),
        Err(e) => thread_option = get_thread_by_slug(&slug_or_id.to_string(), &conn)
    }

    let mut thread;
    match thread_option {
        Ok(d) => thread = d,
        Err(err) => {
            resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found"})).set_mut(status::NotFound);
            return Ok(resp);
        }
    }

    let json_posts: Vec<JsonPost> = serde_json::from_str(&raw).unwrap();

    if json_posts.len() == 0 {
        resp.set_mut(JsonResponse::json(json_posts)).set_mut(status::Created);
        return Ok(resp);
    }

    match p_m::create_posts(&thread, json_posts, &conn) {
        Ok(val) => {
            resp.set_mut(JsonResponse::json(val)).set_mut(status::Created);
            return Ok(resp);
        }
        Err(val) => if (val == 409) {
            resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found thread"})).set_mut(status::Conflict);
            return Ok(resp);
        } else {
            resp.set_mut(JsonResponse::json(ErrorMsg{message: "Not found user"})).set_mut(status::NotFound);
            return Ok(resp);
        }
    }





//    if (posts.isEmpty()) {
//        return new ResponseEntity<>(posts, null, HttpStatus.CREATED);
//    }

//        List<PostModel> postModelList = new ArrayList<>();

//    for (PostModel post : posts) {
//    if (post.getParent() != 0) {
//    PostModel prnt = new PostModel();
//    StatusManagerRequest status1 = postManager.findById(post.getParent(), prnt);
//
//    switch (status1.getCode()) {
//    case NO_RESULT:
//    return new ResponseEntity<>(new ErrorView(status1.getMessage()), null, HttpStatus.CONFLICT);
//    case DB_ERROR:
//    return new ResponseEntity<>(new ErrorView(status1.getMessage()), null, HttpStatus.INTERNAL_SERVER_ERROR);
//    }
//    post.setForum(threadModel.getForum());
//    post.setThread(threadModel.getId());
//
//    if (!threadModel.getId().equals(prnt.getThread())) {
//    return new ResponseEntity<>(new ErrorView("not equals id"), null, HttpStatus.CONFLICT);
//    }
//    }
//    post.setForum(threadModel.getForum());
//    post.setThread(threadModel.getId());
////            postModelList.add(new PostModel(post));
//    }
//
//    StatusManagerRequest status2 = postManager.create(posts, threadModel);
//
//    switch (status2.getCode()) {
//        case CONFILICT:
//        return new ResponseEntity<>(new ErrorView(status2.getMessage()), null, HttpStatus.CONFLICT);
//        case DB_ERROR:
//        return new ResponseEntity<>(new ErrorView(status2.getMessage()), null, HttpStatus.NOT_FOUND);
//    }
//
//    let mut posts: Vec<Post> = Vec::new();
//
////    pub const INSERT_POST: &'static str = "INSERT INTO post(id, parent_id, author_id, created, forum_slug, message, thread_id, id_of_root)\
//
//    for json_post in json_posts {
//        let u_id;
//        match  find_user_id(&json_post.author.unwrap(), &conn) {
//            Ok(id) => u_id = id,
//            Err(err) => {
//                resp.set_mut(JsonResponse::json(ErrorMsg{message: "No such user"})).set_mut(status::Created);
//                return Ok(resp);
//            }
//        }
//        let p_id = conn.query(SELECT_NEXT_POST_ID, &[]).unwrap();
//
//        let post = empty_post();
//        if json_post.parent == None || json_post.parent == Some(0) {
//
//        } else {
//
//        }
//    }
    //    for post in &mut val {
//        post.set_author("f".to_string());
//        println!("{:?}", post);
//
//    }//    let mut r = request.get::<bodyparser::Struct<JsonPost>>();

//    println!("{:?}", val);

    println!("{:?}", thread);
//    let created =     let tz: chrono::DateTime<chrono::Utc> = postgres::types::FromSql::from_sql(&TIMESTAMPTZ, data).unwrap();
//    println!("{}", );
    // TODO check thread
//    created = @@f_man.utc_time(Time.new)
//    arr_for_insert = Array.new
//
//    # how to insert another way
//    post_arr.each do |post|
//    # no errors
//    res = @@p_man.get_id()
//    p_id = res[:data]
//
//    #maybe not need to do it there, db think, insert by name and trigger
//    # res = @@u_man.get_id(post[:author])
//    # if res[:status] == 'NO_RES'
//    #   return render :json => {:m => "no user"}, :status => 404
//    # end
//    # u_id = res[:data][:id]
//    u_name = post[:author]
//
//    if post[:parent] == nil
//    arr_for_insert.push([p_id, 0, u_name, created, thread[:forum_slug], post[:message], thread[:id], p_id, "{" + p_id.to_s + "}"])
//    else
//    res = @@p_man.get(post[:parent])
//    if res[:status] == 'NO_RES'
//    return render :json => {:m => res[:data]}, :status => 409
//    end
//    parent = res[:data]
//
//    if thread[:id] != parent[:thread_id]
//    return render :json => {:m => res[:data]}, :status => 409
//    end
//
//    res = @@p_man.get_path(parent[:id])
//    path = res[:data]
//    idx = path.index(',')
//    if idx == nil
//    idx = path.index('}')
//    end
//    id_root = path[1, idx - 1]
//    path.insert(-2, ", " + p_id.to_s)
//    arr_for_insert.push( [p_id, parent[:id], u_name, created, thread[:forum_slug], post[:message], thread[:id], id_root,  path])
//    end
//
//    post[:created] = created#.slice(-5) + "Z"
//    post[:forum] = forum[:slug]
//    post[:id] = p_id
//    post[:thread] = thread[:id]
//    end
//    res = @@p_man.create(arr_for_insert)
//    if res[:status] == 'NO_RES'
//    return render :json => {:m => res[:data]}, :status => 404
//    end
//    render :json => post_arr, :status => @@status_hash[res[:status]]
//    end


    return Ok(resp);
}

//    println!("{:?}", request.get::<bodyparser::Struct<Raw>>());
//    println!("{}",r);
//    let mut dbForum = empty_forum();
