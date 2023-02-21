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

WebUI.click(findTestObject('Object Repository/Page_Vape Station  Vape Shop at Redbank Plains/a_Shop 60ml E-liquids'))

WebUI.click(findTestObject('Object Repository/Page_E-liquids  VAPE STATION/img_WE DO NOT SELL NICOTINE AS IT IS ILLEGA_6e7cf5'))

WebUI.click(findTestObject('Object Repository/Page_Vape Station  Vape Shop at Redbank Plains/a_Shop 100ml E-Liquids'))

WebUI.click(findTestObject('Object Repository/Page_100ml VS E-LIQUIDS  VAPE STATION/img_WE DO NOT SELL NICOTINE AS IT IS ILLEGA_6e7cf5'))

WebUI.click(findTestObject('Object Repository/Page_Vape Station  Vape Shop at Redbank Plains/a_Shop Interstate  International  E-liquids'))

WebUI.closeBrowser()

