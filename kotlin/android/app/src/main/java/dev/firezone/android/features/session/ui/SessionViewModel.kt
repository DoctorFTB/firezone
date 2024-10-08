/* Licensed under Apache 2.0 (C) 2024 Firezone, Inc. */
package dev.firezone.android.features.session.ui

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import dagger.hilt.android.lifecycle.HiltViewModel
import dev.firezone.android.core.data.Repository
import dev.firezone.android.core.data.ResourceState
import dev.firezone.android.tunnel.TunnelService.Companion.State
import dev.firezone.android.tunnel.model.Resource
import dev.firezone.android.tunnel.model.isInternetResource
import javax.inject.Inject

@HiltViewModel
internal class SessionViewModel
    @Inject
    constructor() : ViewModel() {
        @Inject
        internal lateinit var repo: Repository
        private val _favoriteResourcesLiveData = MutableLiveData<HashSet<String>>(HashSet())
        private val _serviceStatusLiveData = MutableLiveData<State>()
        private val _resourcesLiveData = MutableLiveData<List<Resource>>(emptyList())
        private var showOnlyFavorites: Boolean = false

        val favoriteResourcesLiveData: MutableLiveData<HashSet<String>>
            get() = _favoriteResourcesLiveData
        val serviceStatusLiveData: MutableLiveData<State>
            get() = _serviceStatusLiveData
        val resourcesLiveData: MutableLiveData<List<Resource>>
            get() = _resourcesLiveData

        private val favoriteResources: HashSet<String>
            get() = favoriteResourcesLiveData.value!!

        // Actor name
        fun clearActorName() = repo.clearActorName()

        fun getActorName() = repo.getActorNameSync()

        fun addFavoriteResource(id: String) {
            val value = favoriteResources
            value.add(id)
            repo.saveFavoritesSync(value)
            // Update LiveData
            _favoriteResourcesLiveData.value = value
        }

        fun removeFavoriteResource(id: String) {
            val value = favoriteResources
            value.remove(id)
            repo.saveFavoritesSync(value)
            if (forceAllResourcesTab()) {
                showOnlyFavorites = false
            }
            // Update LiveData
            _favoriteResourcesLiveData.value = value
        }

        fun clearToken() = repo.clearToken()

        // The subset of Resources to actually render
        fun resourcesList(isInternetResourceEnabled: ResourceState): List<ResourceViewModel> {
            val resources =
                resourcesLiveData.value!!.map {
                    if (it.isInternetResource()) {
                        ResourceViewModel(it, isInternetResourceEnabled)
                    } else {
                        ResourceViewModel(it, ResourceState.ENABLED)
                    }
                }

            return if (favoriteResources.isEmpty()) {
                resources
            } else if (showOnlyFavorites) {
                resources.filter { favoriteResources.contains(it.id) }
            } else {
                resources
            }
        }

        fun forceAllResourcesTab(): Boolean {
            return favoriteResources.isEmpty()
        }

        fun showFavoritesTab(): Boolean {
            return favoriteResources.isNotEmpty()
        }

        fun tabSelected(position: Int) {
            showOnlyFavorites =
                when (position) {
                    RESOURCES_TAB_FAVORITES -> {
                        true
                    }

                    RESOURCES_TAB_ALL -> {
                        false
                    }

                    else -> throw IllegalArgumentException("Invalid tab position: $position")
                }
        }

        companion object {
            const val RESOURCES_TAB_FAVORITES = 0
            const val RESOURCES_TAB_ALL = 1
        }
    }
