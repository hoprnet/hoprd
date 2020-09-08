import React, { useEffect, useState } from "react";
import Head from 'next/head'
import styles from '../styles/Home.module.css'
import Logo from '../components/logo'
import { Logs } from '../components/log'
import { Connection } from '../connection'
import dynamic from "next/dynamic";
import { Balance } from '../components/balance'
import { ConnectedPeers } from '../components/connected-peers'

const Jazzicon = dynamic(() => import("../components/jazzicon"), { ssr: false });

let connection

export default function Home() {

  const [selectedTab, setSelectedTab] = useState(0)
  const [connecting, setConnecting] = useState(true);
  const [messages, setMessages] = useState([]); // The fetish for immutability in react means this will be slower than a mutable array..
  const [peers, setConnectedPeers] = useState([]);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      connection = new Connection(setConnecting, setMessages, setConnectedPeers)
      return Connection.disconnect
    }
  }, [])

  return (
    <>
      <Head>
        <title>HOPR Admin</title>
      </Head>

      <Logo
        onClick={() => setSelectedTab((selectedTab + 1) % 3)}
        />

      <div className={styles.container}>
        <h1>HOPR Logs [TESTNET NODE]</h1>
        <div className={styles.tabs}>
          <a href='#'
            className={selectedTab == 0 ? styles.selectedTab : ''}
            onClick={() => setSelectedTab(0)}>Logs</a>
          <a href='#'
            className={selectedTab == 1 ? styles.selectedTab : ''}
            onClick={() => setSelectedTab(1)}>Connected Peers</a>
          <a href='#'
            className={selectedTab == 2 ? styles.selectedTab : ''}
            onClick={() => setSelectedTab(2)}>Balance</a>
        </div>

        <div className={styles.pane}>
          { selectedTab == 0 && <Logs
              messages={messages}
              connecting={connecting}
              connection={connection} /> }
          { selectedTab == 1 && <ConnectedPeers peers={peers} /> }
          { selectedTab == 2 && <Balance /> }
        </div>
      </div>
    </>
  )
}
