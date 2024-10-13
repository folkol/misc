import './App.css';
import {Routes, Route, Outlet, Link, useLocation} from "react-router-dom";
import * as React from 'react';
import Button from '@mui/material/Button';
import {useQuery} from "@tanstack/react-query";
import CssBaseline from '@mui/material/CssBaseline';
import Container from '@mui/material/Container';
import Paper from '@mui/material/Paper';
import {ButtonGroup, CardMedia, Grid2, Pagination, Typography} from "@mui/material";
import Box from '@mui/material/Box';
import {useEffect, useState} from "react";
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import * as PropTypes from "prop-types";

function NavButton({to, children}) {
    const {pathname} = useLocation();
    const textDecoration = to === pathname ? "underline" : "none";
    return <Link style={{"textDecoration": textDecoration}} to={to}><Button>{children}</Button></Link>;
}

function Layout() {
    return (
        <CssBaseline>
            <Box m={2}>
                <Container>
                    <h1>A somewhat broken web page</h1>
                    <ButtonGroup variant="text">
                        <NavButton to="/">Home</NavButton>
                        <NavButton to="/stuff">Stuff</NavButton>
                        <NavButton to="/items">Items</NavButton>
                        <NavButton to="/misc">Misc</NavButton>
                        <NavButton to="/junk">Junk</NavButton>
                    </ButtonGroup>
                    <Paper elevation={5}>
                        <Box p={1}>
                            <Outlet/>
                        </Box>
                    </Paper>
                </Container>
            </Box>
        </CssBaseline>
    );
}

function Home() {
    return (
        <div>
            <h2>Home</h2>
            <p>There are some problems with the other pages, try to identify what the problem is — and come up with a
                suggested fix!</p>
            <h2>Bug reports</h2>
            <h3>Page: Stuff</h3>
            <p>[User]: The page works, and I like the content — but it feels a bit slow and my iPhone becomes very hot
                when I visit the page.</p>
            <h3>Page: Items</h3>
            <p>[Dev]: I changed the code to be JSON, but now it doens't work. I have no idea what the problem is :(</p>
            <h3>Page: Misc</h3>
            <p>[Dev]: It worked yesterday, the backend devs must have changed something. Plz send help!</p>
            <h3>Page: Junk</h3>
            <p>[Dev]: It works on my machine, but it doesn't work in production.</p>
        </div>
    );
}

function GamerCard({name, occupation}) {
    return (
        <Grid2 xs={6}>
            <Box p={1}>
                <Card>
                    <CardMedia
                        sx={{height: 140}}
                        image="/avatar.png"
                        title="True Gamer"
                    />
                    <CardContent>
                        <Typography gutterBottom variant="h5" component="div">
                            Name: {name}
                        </Typography>
                        <Typography variant="body2" sx={{color: "text.secondary"}}>
                            Occupation: {occupation}
                        </Typography>
                    </CardContent>
                </Card>
            </Box>
        </Grid2>
    );
}

function Stuff() {
    const [page, setPage] = useState(1);
    useEffect(() => {

    }, [page]);

    function handleChange(event, n) {
        setPage(n);
    }

    const {isPending, error, data} = useData('stuff', page);

    if (isPending) return 'Loading...';

    if (error) return 'An error has occurred: ' + error.message;

    return (
        <div>
            <h2>Stuff</h2>
            <Pagination count={500} page={page} shape="rounded" onChange={handleChange}/>
            <Grid2 container>
                {data.slice((page - 1) * 10, (page - 1) * 10 + 10).map(item => {
                    return <GamerCard key={item.id} name={item.name} occupation={item.occupation}/>;
                })}
            </Grid2>
        </div>
    )
}

function Items() {
    const {isPending, error, data} = useData('items');


    if (isPending) return 'Loading...';

    if (error) return 'An error has occurred: ' + error.message;

    return (
        <div>
            <h2>{data}</h2>
        </div>
    );
}

function useData(path, queryKey) {
    return useQuery({
        gcTime: 1000,
        queryKey: ['repoData', queryKey],
        retryDelay: 500,
        retry: 1,
        queryFn: () => fetch(`https://backend-bold-forest-9507.fly.dev/${path}`).then((res) => {
            if (res.ok) {
                return res.json();
            } else {
                throw `Problem fetching ${path}`;
            }
        }),
    })
}

function Misc() {
    const {isPending, error, data} = useData('misc');

    if (isPending) return 'Loading...';

    if (error) return 'An error has occurred: ' + error.message;

    return (
        <div>
            <h1>{data.name}</h1>
            <p>{data.description}</p>
            <pre>{JSON.stringify(data, undefined, 2)}</pre>
        </div>
    );
}

export default function App() {
    return (
        <Routes>
            <Route path="/" element={<Layout/>}>
                <Route index element={<Home/>}/>
                <Route path="stuff" element={<Stuff/>}/>
                <Route path="items" element={<Items/>}/>
                <Route path="misc" element={<Misc/>}/>
                <Route path="junk" element={<Junk/>}/>
            </Route>
        </Routes>
    );
}

function Junk() {
    const {isPending, error, data} = useData('junk');

    if (isPending) return 'Loading...';

    if (error) return 'An error has occurred: ' + error.message;

    return (
        <div>
            <h1>{data.name}</h1>
            <p>{data.description}</p>
            <pre>{JSON.stringify(data, undefined, 2)}</pre>
        </div>
    );
}
