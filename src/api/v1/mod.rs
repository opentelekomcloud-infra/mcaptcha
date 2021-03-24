/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use actix_web::web::ServiceConfig;

pub mod auth;
pub mod mcaptcha;
pub mod meta;

pub fn services(cfg: &mut ServiceConfig) {
    // auth
    cfg.service(auth::signout);
    cfg.service(auth::signin);
    cfg.service(auth::signup);
    cfg.service(auth::delete_account);

    // mcaptcha
    // 1. domain and mcaptcha
    cfg.service(mcaptcha::add_domain);
    cfg.service(mcaptcha::delete_domain);
    cfg.service(mcaptcha::add_mcaptcha);
    cfg.service(mcaptcha::delete_mcaptcha);
    // levels
    cfg.service(mcaptcha::add_levels);
    cfg.service(mcaptcha::update_levels);
    cfg.service(mcaptcha::delete_levels);
    cfg.service(mcaptcha::get_levels);
    // duration
    cfg.service(mcaptcha::update_duration);
    cfg.service(mcaptcha::get_duration);

    // meta
    cfg.service(meta::build_details);
    cfg.service(meta::health);
}

#[cfg(test)]
mod tests;