
package com.folkol;

import io.javalin.Javalin;

public class HelloWorld {
  public static void main(String[] args) {
    Javalin app = Javalin.start(7000);
    app.get("/ping", ctx -> ctx.result("pong"));
  }
}
