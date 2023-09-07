package com.folkol;

import jakarta.ws.rs.ApplicationPath;
import jakarta.ws.rs.SeBootstrap;
import jakarta.ws.rs.core.Application;
import org.glassfish.jersey.server.ResourceConfig;

@ApplicationPath("/")
public class Main extends Application {
    public static void main(String[] args) throws InterruptedException {
        ResourceConfig resourceConfig = new ResourceConfig();
        resourceConfig.packages("com.folkol");
        SeBootstrap.start(resourceConfig, SeBootstrap.Configuration.builder().build());

        Thread.currentThread().join();
    }
}