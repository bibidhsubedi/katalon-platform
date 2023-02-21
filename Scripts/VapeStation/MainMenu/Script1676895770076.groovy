import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://vapestation.co/')

WebUI.click(findTestObject('Object Repository/Page_Vape Station  Vape Shop at Redbank Plains/button_Yes'))

WebUI.click(findTestObject('Object Repository/Page_Vape Station  Vape Shop at Redbank Plains/a_60ml VS E-LIQUIDS'))

WebUI.click(findTestObject('Object Repository/Page_Nicotine Script  VAPE STATION/a_100ml VS E-LIQUIDS'))

WebUI.click(findTestObject('Object Repository/Page_100ml VS E-LIQUIDS  VAPE STATION/a_INTERSTATE  INTERNATIONAL E-LIQUIDS'))

WebUI.click(findTestObject('Object Repository/Page_INTERSTATE AND INTERNATIONAL E-LIQUID _f73633/a_COILS'))

WebUI.click(findTestObject('Object Repository/Page_COILS  VAPE STATION/a_KITS'))

WebUI.click(findTestObject('Object Repository/Page_COMPLETE KITS  VAPE STATION/a_MODS'))

WebUI.click(findTestObject('Object Repository/Page_MODS  VAPE STATION/a_TANKS'))

WebUI.click(findTestObject('Object Repository/Page_Tanks  VAPE STATION/a_BATTERIES  CHARGERS'))

WebUI.click(findTestObject('Object Repository/Page_BATTERIES  CHARGERS  VAPE STATION/a_REPLACEMENT GLASS'))

WebUI.click(findTestObject('Object Repository/Page_GLASS  VAPE STATION/a_VS STANDARD CONCENTRATES'))

WebUI.click(findTestObject('Object Repository/Page_VS Standard Concentrates  VAPE STATION/a_STORE LOCATOR'))

WebUI.click(findTestObject('Object Repository/Page_Vapestation Store Locator  VAPE STATION/a_BLOGS'))

WebUI.click(findTestObject('Object Repository/Page_New blogs VAPE STATION/a_Nicotine Script'))

WebUI.click(findTestObject('Object Repository/Page_Nicotine Script  VAPE STATION/a_ADULT PRODUCTS'))

WebUI.closeBrowser()

