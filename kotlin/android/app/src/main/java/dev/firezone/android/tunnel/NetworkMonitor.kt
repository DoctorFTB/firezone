/* Licensed under Apache 2.0 (C) 2024 Firezone, Inc. */
import android.net.ConnectivityManager
import android.net.LinkProperties
import android.net.Network
import com.google.gson.Gson
import dev.firezone.android.tunnel.ConnlibSession
import dev.firezone.android.tunnel.TunnelService
import dev.firezone.android.tunnel.TunnelStatusNotification
import java.net.InetAddress

class NetworkMonitor(private val tunnelService: TunnelService) : ConnectivityManager.NetworkCallback() {
    private var lastNetwork: Network? = null
    private var lastDns: List<InetAddress>? = null

    override fun onLinkPropertiesChanged(
        network: Network,
        linkProperties: LinkProperties,
    ) {
        // Acquire mutex lock
        if (tunnelService.lock.tryLock()) {
            if (tunnelService.tunnelState != TunnelService.Companion.State.UP) {
                tunnelService.tunnelState = TunnelService.Companion.State.UP
                tunnelService.updateStatusNotification(TunnelStatusNotification.Connected)
            }

            if (lastDns != linkProperties.dnsServers) {
                lastDns = linkProperties.dnsServers

                // Strip the scope id from IPv6 addresses. See https://github.com/firezone/firezone/issues/5781
                val dnsList = linkProperties.dnsServers.map { it.hostAddress!!.split("%")[0] }
                ConnlibSession.setDns(tunnelService.connlibSessionPtr!!, Gson().toJson(dnsList))
            }

            if (lastNetwork != network) {
                lastNetwork = network
                ConnlibSession.reconnect(tunnelService.connlibSessionPtr!!)
            }

            // Release mutex lock
            tunnelService.lock.unlock()
        }

        super.onLinkPropertiesChanged(network, linkProperties)
    }
}
