import styles from '../styles/connected-peers.module.css'
import dynamic from "next/dynamic";

const Jazzicon = dynamic(() => import("../components/jazzicon"), { ssr: false });

export function ConnectedPeers({ peers }){
  return (
    <div className={styles.connectedPeers}>
      <h2>Connected Peers ({peers.length})</h2>
      <div className={styles.connectedPeersList}>
        { peers.map( x => (
          <div className={styles.peer} key={x}>
            <Jazzicon
              diameter={40}
              address={x}
              className={styles.peerIcon}
            />
            <div>{x}</div>
          </div>
        )) }
      </div>
    </div>
  )
}
