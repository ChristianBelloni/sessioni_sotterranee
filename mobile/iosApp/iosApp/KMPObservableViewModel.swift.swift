//
//  KMPObservableViewModel.swift.swift
//  iosApp
//
//  Created by Christian Belloni on 07/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//

import KMPObservableViewModelCore
import Shared // This should be your shared KMP module

extension Kmp_observableviewmodel_coreViewModel: @retroactive ObservableObject {}
extension Kmp_observableviewmodel_coreViewModel: @retroactive ViewModel { }
