import styles from '../styles/connected-peers.module.css'
import dynamic from "next/dynamic";

const Jazzicon = dynamic(() => import("../components/jazzicon"), { ssr: false });

export function ConnectedPeers({ peers }){
  return (
    <div className={styles.connectedPeers}>
      <table>
        <tr>
          <th colspan={2}>Peer</th>
          <th>Channel</th>
        </tr>
        { peers.map( x => (
          <tr className={styles.peer} key={x}>
            <td>
              <Jazzicon
                diameter={40}
                address={x}
                className={styles.peerIcon}
              />
            </td>
            <td>{x}</td>
            <td></td>
          </tr>
        )) }
      </table>
    </div>
  )
}
